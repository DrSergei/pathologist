use crate::{Token, TokenKind};
use indexmap::IndexMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub enum MacroDef {
    Object {
        replacement: Vec<Token>,
    },
    Function {
        params: Vec<String>,
        replacement: Vec<Token>,
        variadic: bool,
    },
}

/// One executed macro directive, in program order. Cached include entries
/// record these so replay reproduces the header's effects exactly — a
/// state diff cannot represent a no-op `#undef` (name absent at capture,
/// present in a later consumer) or `#undef X` + `#define X new` of a name
/// that existed at both capture boundaries.
#[derive(Debug, Clone)]
pub enum MacroOp {
    Define(String, MacroDef),
    Undef(String),
}

pub type MacroTable = IndexMap<String, MacroDef>;
pub type SharedMacroTable = Arc<RwLock<MacroTable>>;

pub fn new_shared_macro_table() -> SharedMacroTable {
    Arc::new(RwLock::new(MacroTable::new()))
}

pub fn macro_table_from_defines(defines: &indexmap::IndexMap<String, String>) -> MacroTable {
    let mut table = MacroTable::new();
    for (name, val) in defines {
        table.insert(
            name.clone(),
            MacroDef::Object {
                replacement: lex_macro_body(val),
            },
        );
    }
    table
}

/// Tokenize a macro replacement list from source text (Eof stripped).
pub(crate) fn lex_macro_body(src: &str) -> Vec<Token> {
    crate::Lexer::new(src)
        .tokenize()
        .into_iter()
        .filter(|t| !matches!(t.kind, TokenKind::Eof))
        .collect()
}
