use crate::analyzer::Issue;
use std::collections::HashSet;
use std::path::PathBuf;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Query, QueryCursor, Tree};

pub fn check_sql_taint(
    source_code: &str,
    tree: &Tree,
    file_path: &PathBuf,
    language: Language,
) -> Vec<Issue> {
    let mut issues = Vec::new();

    let mut tainted_vars: HashSet<String> = HashSet::new();

    // We look for any variable declaration
    let source_query = Query::new(
        &language,
        r#"
        (variable_declarator
            name: (identifier) @var_name
            value: (_) @var_value
        )
    "#,
    )
    .expect("Failed to compile source query");

    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&source_query, tree.root_node(), source_code.as_bytes());

    while let Some(m) = matches.next() {
        let mut var_name = "";
        let mut var_value_text = "";

        for capture in m.captures {
            let capture_name = source_query.capture_names()[capture.index as usize];
            if capture_name == "var_name" {
                var_name = &source_code[capture.node.byte_range()];
            } else if capture_name == "var_value" {
                var_value_text = &source_code[capture.node.byte_range()];
            }
        }

        // TAINT LOGIC: If the value string contains "req.query" or "req.body",
        // we consider the variable name tainted!
        if var_value_text.contains("req.query") || var_value_text.contains("req.body") {
            tainted_vars.insert(var_name.to_string());
        }
    }

    // ==========================================
    // PASS 2: FIND SINKS (Database Execution)
    // ==========================================
    // We look for db.query(arg)
    let sink_query = Query::new(
        &language,
        r#"
        (call_expression
            function: (member_expression
                property: (property_identifier) @method_name (#eq? @method_name "query")
            )
            arguments: (arguments (identifier) @arg_name)
        ) @issue_node
    "#,
    )
    .expect("Failed to compile sink query");

    let mut cursor2 = QueryCursor::new();
    let mut matches2 = cursor2.matches(&sink_query, tree.root_node(), source_code.as_bytes());

    while let Some(m) = matches2.next() {
        let mut arg_name = "";
        let mut issue_node = None;

        for capture in m.captures {
            let capture_name = sink_query.capture_names()[capture.index as usize];
            if capture_name == "arg_name" {
                arg_name = &source_code[capture.node.byte_range()];
            } else if capture_name == "issue_node" {
                issue_node = Some(capture.node);
            }
        }

        // THE MAGIC: Is the argument passed to db.query in our list of tainted variables?
        if tainted_vars.contains(arg_name) {
            if let Some(node) = issue_node {
                issues.push(Issue {
                    rule_id: "sql-injection-taint".to_string(),
                    severity: "Critical".to_string(),
                    file: file_path.clone(),
                    line: node.start_position().row + 1,
                    message: format!("SQL Injection detected! Tainted variable '{}' flows directly into a database query.", arg_name),
                });
            }
        }
    }

    issues
}
