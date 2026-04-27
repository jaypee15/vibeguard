use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub id: String,
    pub languages: Vec<String>,
    pub severity: String,
    pub message: String,
    pub query: String,
    pub fix_guidance: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RuleConfig {
    pub rules: Vec<Rule>,
}

pub fn load_rules() -> Vec<Rule> {
    let yaml_content = include_str!("../rules/rules.yaml");
    let config: RuleConfig =
        serde_yaml::from_str(yaml_content).expect("Failed to parse rules file");

    config.rules
}
