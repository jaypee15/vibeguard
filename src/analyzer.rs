use std::path::PathBuf;
use tree_sitter::{Query, QueryCursor,Node};
use streaming_iterator::StreamingIterator;

#[derive(Debug)]
pub struct Issue {
    pub rule_id: String,
    pub file: PathBuf,
    pub line: usize,
    pub message: String,
}

pub fn analyze_javascript(source_code: &str, tree: &tree_sitter::Tree, file_path: &PathBuf) -> Vec<Issue> {
    let mut issues = Vec::new();
    let query_str = r#"
        (call_expression
            function: (identifier) @func_name
        (#eq? @func_name "eval")
    ) @eval_call 
    "#;
    
    let language = tree_sitter_javascript::LANGUAGE.into();
    let query = Query::new(&language, query_str).expect("Invalid Tree sitter query");
    
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());
    while let Some(m) = matches.next() {
        for capture in m.captures {
            let capture_name = query.capture_names()[capture.index as usize];
            
            if capture_name == "eval_call" {
                let node: Node = capture.node;
                
                issues.push(Issue {
                    rule_id: "insecure-eval". to_string(),
                    file: file_path.clone(),
                    line: node.start_position().row + 1,
                    message: "Found usage of `eval()`. This can lead to Remote code Execution.".to_string(),
                });
            }
            
        }
    }
    issues
}