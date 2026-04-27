use crate::rule_engine::Rule;
use serde::Serialize;
use std::path::PathBuf;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Query, QueryCursor};

#[derive(Debug, Serialize)]
pub struct Issue {
    pub rule_id: String,
    pub severity: String,
    pub file: PathBuf,
    pub line: usize,
    pub message: String,
    pub fix_guidance: Option<String>,
}

pub fn analyze_code(
    source_code: &str,
    tree: &tree_sitter::Tree,
    file_path: &PathBuf,
    rules: &[Rule],
    language: Language,
) -> Vec<Issue> {
    let mut issues = Vec::new();

    for rule in rules {
        let query = match Query::new(&language, &rule.query) {
            Ok(q) => q,
            Err(e) => {
                println!("Warning: Invalid query for rule {}: {}", rule.id, e);
                continue;
            }
        };
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());
        while let Some(m) = matches.next() {
            for capture in m.captures {
                let capture_name = query.capture_names()[capture.index as usize];

                if capture_name == "issue" {
                    let node: Node = capture.node;

                    issues.push(Issue {
                        rule_id: rule.id.clone(),
                        severity: rule.severity.clone(),
                        file: file_path.clone(),
                        line: node.start_position().row + 1,
                        message: rule.message.clone(),
                        fix_guidance: rule.fix_guidance.clone(),
                    });
                }
            }
        }
    }

    issues
}
