use tree_sitter::{Parser, Tree};

pub fn parse_javascript(source_code: &str) -> Option<Tree> {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .expect("Error loading javascript grammer");
    parser.parse(source_code, None)
}
