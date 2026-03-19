use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub id: String,
    pub severity: String,
    pub message: String,
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    pub rules: Vec<Rule>,
}

pub fn load_rules(file_path: &str) -> Vec<Rule> {
    let yaml_content = fs::read_to_string(file_path)
        .expect(&format!("Failed to read rules file at {}", file_path));
    let config: RuleConfig =
        serde_yaml::from_str(&yaml_content).expect("Failed to parse rules file");

    config.rules
}
