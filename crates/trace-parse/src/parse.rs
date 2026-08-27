use std::cell::RefCell;
use std::sync::Arc;
use tree_sitter::{Node, Parser, Tree};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLang {
    C,
    Cpp,
}

impl SourceLang {
    pub fn from_path(path: &std::path::Path) -> Self {
        if crate::discover::is_cpp_path(path) || crate::discover::is_cpp_header_path(path) {
            SourceLang::Cpp
        } else {
            SourceLang::C
        }
    }
}

thread_local! {
    static PARSER_C: RefCell<Parser> = RefCell::new(make_parser(SourceLang::C));
    static PARSER_CPP: RefCell<Parser> = RefCell::new(make_parser(SourceLang::Cpp));
}

fn make_parser(lang: SourceLang) -> Parser {
    let mut parser = Parser::new();
    match lang {
        SourceLang::C => parser
            .set_language(&tree_sitter_c::LANGUAGE.into())
            .expect("failed to set C language"),
        SourceLang::Cpp => parser
            .set_language(&tree_sitter_cpp::LANGUAGE.into())
            .expect("failed to set C++ language"),
    }
    parser
}

pub struct ParseResult {
    pub tree: Tree,
    pub source: Arc<str>,
}

/// Parse with the grammar matching `lang`. The C++ grammar is a superset of
/// the C grammar's node vocabulary for everything the lowering matches on, so
/// existing C paths are unaffected.
pub fn parse_source_with_lang(
    source: impl Into<Arc<str>>,
    lang: SourceLang,
) -> Result<ParseResult, String> {
    let source: Arc<str> = source.into();
    let tree = match lang {
        SourceLang::C => PARSER_C.with(|p| p.borrow_mut().parse(source.as_ref(), None)),
        SourceLang::Cpp => PARSER_CPP.with(|p| p.borrow_mut().parse(source.as_ref(), None)),
    };
    let tree = tree.ok_or_else(|| "tree-sitter returned no tree".to_string())?;
    Ok(ParseResult { tree, source })
}

pub fn parse_c_source(source: impl Into<Arc<str>>) -> Result<ParseResult, String> {
    parse_source_with_lang(source, SourceLang::C)
}

pub fn node_text<'a>(source: &'a str, node: &Node) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

pub fn has_parse_errors(tree: &Tree) -> bool {
    tree.root_node().has_error()
}
