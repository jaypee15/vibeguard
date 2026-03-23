use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

use crate::analyzer::{Issue, analyze_javascript};
use crate::parser::parse_javascript;
use crate::rule_engine::load_rules;

pub fn get_files_to_scan(dir: &str) -> Vec<PathBuf> {
    let walker = WalkBuilder::new(dir).build();

    walker
        .filter_map(|result| result.ok())
        .filter(|entry| {
            let is_file = entry.file_type().map_or(false, |ft| ft.is_file());
            if !is_file {
                return false;
            }

            if let Some(ext) = entry.path().extension() {
                let ext_str = ext.to_string_lossy();
                matches!(ext_str.as_ref(), "js" | "ts" | "tsx" | "jsx" | "py")
            } else {
                false
            }
        })
        .map(|entry| entry.into_path())
        .collect()
}

pub fn run_scan(path: &str) -> Vec<Issue> {
    let rules = load_rules("rules/javascript.yaml");
    let files = get_files_to_scan(path);

    files
        .par_iter()
        .flat_map(|file| {
            let mut file_issues = Vec::new();

            if let Some(ext) = file.extension() {
                if ext == "js" {
                    if let Ok(content) = fs::read_to_string(file) {
                        if let Some(tree) = parse_javascript(&content) {
                            file_issues = analyze_javascript(&content, &tree, file, &rules);
                        }
                    }
                }
            }

            file_issues
        })
        .collect()
}
