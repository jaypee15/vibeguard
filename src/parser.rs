use tree_sitter::{Language, Parser, Tree};

pub fn parse_code(source_code: &str, language: Language) -> Option<Tree> {
    let mut parser = Parser::new();
    if parser.set_language(&language).is_err() {
        return None;
    }
    parser.parse(source_code, None)
}
