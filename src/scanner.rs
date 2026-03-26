use ignore::WalkBuilder;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

use crate::analyzer::{Issue, analyze_code};
use crate::parser::parse_code;
use crate::rule_engine::load_rules;
use crate::taint::check_sql_taint;

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
    let all_rules = load_rules();
    let files = get_files_to_scan(path);

    files
        .par_iter()
        .flat_map(|file| {
            let mut file_issues = Vec::new();

            if let Some(ext) = file.extension().and_then(|e| e.to_str()) {
                let (lang_name, ts_language): (&str, tree_sitter::Language) = match ext {
                    "js" | "jsx" => ("javascript", tree_sitter_javascript::LANGUAGE.into()),
                    "ts" => (
                        "typescript",
                        tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
                    ),
                    "tsx" => ("typescript", tree_sitter_typescript::LANGUAGE_TSX.into()),
                    _ => return Vec::new(),
                };
                let applicable_rules: Vec<_> = all_rules
                    .iter()
                    .filter(|r| r.languages.iter().any(|l| l == lang_name))
                    .cloned()
                    .collect();

                if !applicable_rules.is_empty() {
                    if let Ok(content) = fs::read_to_string(file) {
                        if let Some(tree) = parse_code(&content, ts_language.clone()) {
                            file_issues = analyze_code(
                                &content,
                                &tree,
                                file,
                                &applicable_rules,
                                ts_language.clone(),
                            );
                            let taint_issues = check_sql_taint(&content, &tree, file, ts_language);
                            file_issues.extend(taint_issues);
                        }
                    }
                }
            }

            file_issues
        })
        .collect()
}
