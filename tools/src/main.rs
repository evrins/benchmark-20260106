use crate::entity::{BenResult, default_results};
use charts_rs::{BarChart, Color, THEME_ANT, svg_to_png};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
use std::path::Path;

mod entity;

#[derive(Parser)]
#[command(name = "tools")]
#[command(about = "A benchmarking tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a chart from benchmark results
    #[command(arg_required_else_help = true)]
    CreateChart {
        /// Path to the JSON file containing benchmark results
        #[arg(short, long)]
        file: String,
        #[arg(short, long, default_value = "results.png")]
        output: String,
    },

    /// Create default benchmark results file
    CreateDefault {
        /// Output file path (default: ben-results.json)
        #[arg(short, long, default_value = "ben-results.json")]
        output: String,
    },

    /// Run all benchmarks automatically and generate chart
    RunBenchmarks {
        /// Output file for benchmark results (default: ben-results.json)
        #[arg(short, long, default_value = "ben-results.json")]
        output: String,
        /// Output file for chart (default: results.png)
        #[arg(long, default_value = "results.png")]
        chart_output: String,
        /// Port to run benchmarks on (default: 8080)
        #[arg(long, default_value = "8080")]
        port: u16,
    },

    /// Run benchmark for a specific framework
    RunBenchmark {
        /// Framework to benchmark (e.g., gin, echo, actix-web, etc.)
        #[arg(short, long)]
        framework: String,
        /// Port to run benchmark on (default: 8080)
        #[arg(long, default_value = "8080")]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::CreateChart { file, output } => {
            create_chart(file, output);
            println!("Chart created successfully from {} to {}", file, output);
        }
        Commands::CreateDefault { output } => {
            create_default_ben_result(output);
            println!(
                "Default benchmark results created successfully at {}",
                output
            );
        }
        Commands::RunBenchmarks {
            output,
            chart_output,
            port,
        } => {
            run_all_benchmarks(output, chart_output, *port);
        }
        Commands::RunBenchmark { framework, port } => {
            run_single_benchmark(framework, *port);
        }
    }
}

fn create_chart<T: AsRef<Path>>(fp: T, output: T) {
    let content = read_to_string(fp).unwrap();
    let mut rs: Vec<BenResult> = serde_json::from_str(&content).unwrap();

    rs.sort_by(|a, b| a.lang.cmp(&b.lang).then(a.framework.cmp(&b.framework)));

    let mut bar_chart = BarChart::new_with_theme(
        vec![("QPS", rs.iter().map(|it| it.qps).collect()).into()],
        rs.iter().map(|it| it.framework.clone()).collect(),
        THEME_ANT,
    );

    let mut langs: Vec<String> = rs.iter().map(|it| it.lang.clone()).collect();
    langs.sort();
    langs.dedup();

    let lang_map: HashMap<String, usize> = langs
        .iter()
        .enumerate()
        .map(|(i, v)| (v.to_owned(), i))
        .collect();

    let colors: Vec<Option<Color>> = rs
        .iter()
        .map(|it| color_by_lang(&it.lang, &lang_map))
        .collect();

    bar_chart.width = 1000.0;
    bar_chart.series_list[0].colors = Some(colors);
    bar_chart.series_list[0].label_show = true;

    let png = svg_to_png(&bar_chart.svg().unwrap()).unwrap();

    fs::write(output, png).unwrap();
}

fn create_default_ben_result(output: &str) {
    let default_ben_result = default_results();
    let content = serde_json::to_string_pretty(&default_ben_result).unwrap();
    fs::write(output, content).unwrap();
}

fn color_by_lang(lang: &str, lang_map: &HashMap<String, usize>) -> Option<Color> {
    let i = lang_map.get(lang)?;

    let c = colorous::COOL.eval_rational(*i, lang_map.len());

    Some(c.as_tuple().into())
}

use crossbeam_channel as channel;
use regex::Regex;
use std::process;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn run_all_benchmarks(output: &str, chart_output: &str, port: u16) {
    let frameworks = vec![
        "gin",
        "echo",
        "fiber",
        "std",
        "actix-web",
        "axum",
        "spring-boot",
        "quarkus",
        "express",
        "fastify",
        "fastapi",
        "bun",
    ];

    let mut results = Vec::new();

    for framework in frameworks {
        println!("Running benchmark for {}...", framework);
        match run_benchmark_for_framework(framework, port) {
            Ok(result) => {
                println!("Completed benchmark for {}: {} QPS", framework, result.qps);
                results.push(result);
            }
            Err(e) => {
                eprintln!("Failed to benchmark {}: {}", framework, e);
            }
        }
    }

    // Save results to JSON file
    let content = serde_json::to_string_pretty(&results).unwrap();
    std::fs::write(output, content).unwrap();
    println!("Benchmark results saved to {}", output);

    // Generate chart
    create_chart(output, chart_output);
    println!("Chart generated at {}", chart_output);
}

fn run_single_benchmark(framework: &str, port: u16) {
    println!("Running benchmark for {}...", framework);

    match run_benchmark_for_framework(framework, port) {
        Ok(result) => {
            println!("Benchmark result for {}: {} QPS", framework, result.qps);

            // Save single result
            let results = vec![result];
            let content = serde_json::to_string_pretty(&results).unwrap();
            std::fs::write("single-benchmark-result.json", content).unwrap();
            println!("Single benchmark result saved to single-benchmark-result.json");
        }
        Err(e) => {
            eprintln!("Failed to benchmark {}: {}", framework, e);
        }
    }
}

fn run_benchmark_for_framework(framework: &str, port: u16) -> Result<BenResult, String> {
    // Start the server in a separate thread
    let server_cmd = format!("just run-{}", framework);
    let (tx, rx) = channel::unbounded();
    let (stop_tx, stop_rx) = channel::unbounded();
    let server_tx = tx.clone();

    let server_thread = thread::spawn(move || {
        #[cfg(unix)]
        use std::os::unix::process::CommandExt;
        
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c")
            .arg(&server_cmd)
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped());
            
        #[cfg(unix)]
        cmd.process_group(0); // Create new process group on Unix
        
        let mut child = cmd.spawn().expect("Failed to start server");
        let child_pid = child.id();

        // Wait a bit for the server to start
        thread::sleep(Duration::from_secs(2));

        // Send signal that server is ready
        server_tx.send(true).unwrap();

        // Wait for stop signal or for process to finish
        match stop_rx.recv_timeout(Duration::from_secs(20)) {
            Ok(_) => {
                // Stop signal received, kill the process group/tree
                println!("Stopping server process for PID: {}", child_pid);
                
                #[cfg(unix)]
                {
                    // Kill the entire process group to ensure all child processes are terminated
                    unsafe {
                        libc::killpg(child_pid as i32, libc::SIGTERM);
                        thread::sleep(Duration::from_millis(500));
                        libc::killpg(child_pid as i32, libc::SIGKILL);
                    }
                }
                
                // Also kill the direct child process
                if let Err(e) = child.kill() {
                    eprintln!("Failed to kill direct child process: {}", e);
                } else {
                    println!("Successfully sent kill signal to direct process");
                }
            }
            Err(err) => {
                println!("Failed to stop server: {}", err);
                
                #[cfg(unix)]
                {
                    // Try to kill the process group anyway
                    unsafe {
                        libc::killpg(child_pid as i32, libc::SIGTERM);
                        thread::sleep(Duration::from_millis(500));
                        libc::killpg(child_pid as i32, libc::SIGKILL);
                    }
                }
                
                let _ = child.kill();
            }
        }

        let _ = child.wait();
    });

    // Wait for server to start
    rx.recv().unwrap();

    // Wait a bit more to ensure server is fully ready
    thread::sleep(Duration::from_secs(1));

    // Run the benchmark
    let benchmark_result = run_wrk_benchmark(framework, port);

    // Send stop signal to terminate the server
    let _ = stop_tx.send(());

    // Wait a bit for server to stop gracefully
    thread::sleep(Duration::from_secs(1));

    // Ensure the thread finishes
    let _ = server_thread.join();

    // Additional cleanup: kill any remaining processes on the port
    cleanup_port_processes(port);

    benchmark_result
}

fn run_wrk_benchmark(framework: &str, port: u16) -> Result<BenResult, String> {
    let url = format!("http://localhost:{}", port);
    let wrk_cmd = format!("wrk -t2 -c40 -d15s {}", url);

    let output = Command::new("sh")
        .arg("-c")
        .arg(&wrk_cmd)
        .output()
        .map_err(|e| format!("Failed to execute wrk command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "wrk benchmark failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse wrk output: {}", e))?;

    // Parse QPS from wrk output
    // Example wrk output: "Requests/sec:  123456.78"
    let re = Regex::new(r"Requests/sec:\s*([\d.]+)").unwrap();
    let qps = if let Some(captures) = re.captures(&stdout) {
        captures[1].parse::<f32>().unwrap_or(0.0)
    } else {
        return Err("Failed to parse QPS from wrk output".to_string());
    };

    // Determine language based on framework
    let lang = match framework {
        "gin" | "echo" | "fiber" | "std" => "go".to_string(),
        "actix-web" | "axum" => "rust".to_string(),
        "spring-boot" | "quarkus" => "java".to_string(),
        "express" | "fastify" => "nodejs".to_string(),
        "fastapi" => "python".to_string(),
        "bun" => "bun".to_string(),
        _ => "unknown".to_string(),
    };

    Ok(BenResult {
        lang,
        framework: framework.to_string(),
        qps,
    })
}

fn cleanup_port_processes(port: u16) {
    // Find and kill any processes still using the port
    #[cfg(unix)]
    {
        let lsof_cmd = format!("lsof -ti:{}", port);
        
        if let Ok(output) = Command::new("sh")
            .arg("-c")
            .arg(&lsof_cmd)
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Ok(pid) = line.trim().parse::<i32>() {
                    println!("Killing remaining process on port {}: PID {}", port, pid);
                    unsafe {
                        libc::kill(pid, libc::SIGTERM);
                        thread::sleep(Duration::from_millis(100));
                        libc::kill(pid, libc::SIGKILL);
                    }
                }
            }
        }
    }
    
    #[cfg(windows)]
    {
        // On Windows, use netstat and taskkill
        let netstat_cmd = format!("netstat -ano | findstr :{}", port);
        
        if let Ok(output) = Command::new("cmd")
            .arg("/C")
            .arg(&netstat_cmd)
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(pid_str) = parts.last() {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        println!("Killing remaining process on port {}: PID {}", port, pid);
                        let _ = Command::new("taskkill")
                            .arg("/F")
                            .arg("/PID")
                            .arg(&pid.to_string())
                            .output();
                    }
                }
            }
        }
    }
}
