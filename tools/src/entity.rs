use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BenResult {
    pub lang: String,
    pub framework: String,
    pub qps: f32,
}

pub fn default_results() -> Vec<BenResult> {
    vec![
        BenResult {
            lang: "rust".into(),
            framework: "actix-web".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "rust".into(),
            framework: "axum".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "python".into(),
            framework: "fastapi".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "go".into(),
            framework: "std".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "go".into(),
            framework: "echo".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "go".into(),
            framework: "gin".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "go".into(),
            framework: "fiber".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "nodejs".into(),
            framework: "express".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "nodejs".into(),
            framework: "fastify".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "bun".into(),
            framework: "bun".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "java".into(),
            framework: "spring-boot".into(),
            ..BenResult::default()
        },
        BenResult {
            lang: "java".into(),
            framework: "quarkus".into(),
            ..BenResult::default()
        },
    ]
}
