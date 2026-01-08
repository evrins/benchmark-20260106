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
