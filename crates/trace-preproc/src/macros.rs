use crate::{Language, Token, TokenKind};
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
        /// Invariant: when true, the LAST entry of `params` is the variadic
        /// collector — parse_macro_param_list pushes `"__VA_ARGS__"` for the
        /// anonymous `...` form. A hand-built variadic def (builtins, tests)
        /// must uphold this or the last named parameter will swallow every
        /// argument; substitute_macro debug_asserts it.
        variadic: bool,
    },
    /// Last-resort parser recovery for a gMock declaration macro
    /// (`MOCK_METHOD`, `MOCK_METHODn`, `MOCK_CONST_METHODn`, `…_T`): the
    /// invocation becomes the member prototype it declares. A replacement
    /// list cannot express this — the legacy forms carry the whole
    /// signature in one argument and the modern form parenthesizes
    /// comma-containing return types — so the preprocessor expands it in
    /// code (`expand_gmock_method`). Only the builtin fallback table
    /// creates one.
    GmockMethod,
}

impl MacroDef {
    /// This definition re-lexed as `language`. A replacement list is a
    /// token sequence, and the C and C++ lexers disagree on raw strings,
    /// ud-suffixes and the `->*` punctuator — `R"(x)"` is one C++ token but
    /// `R` + `"(x)"` in C, `'a'C` one C++ token but `'a'` + `C` in C, and
    /// `->*` one C++ token but `->` + `*` in C — so a definition lexed for
    /// one language must not reach a unit of the other as is. The
    /// tokens are spelled back with their adjacency intact, which is all
    /// the two lexers disagree about, and lexed again.
    pub fn relexed(&self, language: Language) -> MacroDef {
        let relex = |tokens: &[Token]| {
            let spelling = crate::preprocessor::spell_tokens(tokens, str::to_string);
            lex_macro_body(&spelling, language)
        };
        match self {
            MacroDef::Object { replacement } => MacroDef::Object {
                replacement: relex(replacement),
            },
            MacroDef::Function {
                params,
                replacement,
                variadic,
            } => MacroDef::Function {
                params: params.clone(),
                replacement: relex(replacement),
                variadic: *variadic,
            },
            MacroDef::GmockMethod => MacroDef::GmockMethod,
        }
    }
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

/// Object-like macros for command-line `-D` definitions, their bodies
/// lexed as `language` (see [`Language`] for what differs).
pub fn macro_table_from_defines(
    defines: &indexmap::IndexMap<String, String>,
    language: Language,
) -> MacroTable {
    let mut table = MacroTable::new();
    for (name, val) in defines {
        table.insert(
            name.clone(),
            MacroDef::Object {
                replacement: lex_macro_body(val, language),
            },
        );
    }
    table
}

/// Tokenize a macro replacement list from source text (Eof stripped).
pub(crate) fn lex_macro_body(src: &str, language: Language) -> Vec<Token> {
    crate::Lexer::new(src, language)
        .tokenize()
        .into_iter()
        .filter(|t| !matches!(t.kind, TokenKind::Eof))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(def: &MacroDef) -> Vec<TokenKind> {
        match def {
            MacroDef::Object { replacement } | MacroDef::Function { replacement, .. } => {
                replacement.iter().map(|t| t.kind.clone()).collect()
            }
            MacroDef::GmockMethod => Vec::new(),
        }
    }

    fn object(src: &str, language: Language) -> MacroDef {
        MacroDef::Object {
            replacement: lex_macro_body(src, language),
        }
    }

    #[test]
    fn relexed_splits_and_rejoins_the_cpp_only_pointer_to_member_token() {
        // `->*` is a C++ punctuator only (#37), so it joins raw strings and
        // ud-suffixes as a shape the two lexers disagree about — which is
        // precisely what `relexed` exists to reconcile when a header is
        // reachable from both a C and a C++ unit.
        let cpp = object("p->*m", Language::Cpp);
        assert_eq!(
            kinds(&cpp),
            vec![
                TokenKind::Identifier("p".to_string()),
                TokenKind::Punct("->*"),
                TokenKind::Identifier("m".to_string()),
            ]
        );
        let as_c = cpp.relexed(Language::C);
        assert_eq!(
            kinds(&as_c),
            vec![
                TokenKind::Identifier("p".to_string()),
                TokenKind::Punct("->"),
                TokenKind::Punct("*"),
                TokenKind::Identifier("m".to_string()),
            ]
        );
        // And back: the round trip re-joins them, so neither direction is lossy.
        assert_eq!(kinds(&as_c.relexed(Language::Cpp)), kinds(&cpp));
    }

    #[test]
    fn relexed_changes_only_the_language_dependent_token_shapes() {
        let ident = |s: &str| TokenKind::Identifier(s.to_string());
        let string = |s: &str| TokenKind::String(s.to_string());
        let chr = |s: &str| TokenKind::Char(s.to_string());
        // C -> C++: `R` glued to `"(x)"` becomes one raw string; `'a'` glued
        // to `C` one user-defined literal.
        let c = object("R\"(x)\"[0] + 'a'C", Language::C);
        assert_eq!(kinds(&c)[..2], [ident("R"), string("\"(x)\"")]);
        let cpp = c.relexed(Language::Cpp);
        assert_eq!(
            kinds(&cpp),
            vec![
                string("R\"(x)\""),
                TokenKind::Punct("["),
                TokenKind::Number("0".into()),
                TokenKind::Punct("]"),
                TokenKind::Punct("+"),
                chr("'a'C"),
            ]
        );
        // C++ -> C: the round trip splits them again.
        assert_eq!(kinds(&cpp.relexed(Language::C)), kinds(&c));
        // Whitespace in the source keeps tokens apart in both languages.
        let spaced = object("R \"(x)\" 'a' C", Language::C);
        assert_eq!(kinds(&spaced.relexed(Language::Cpp)), kinds(&spaced));
        // Same-language re-lexing is the identity.
        assert_eq!(kinds(&c.relexed(Language::C)), kinds(&c));
    }

    #[test]
    fn relexed_keeps_function_like_shape_and_operators() {
        let def = MacroDef::Function {
            params: vec!["x".into(), "__VA_ARGS__".into()],
            replacement: lex_macro_body("#x ## _t(__VA_ARGS__) R\"(x)\"", Language::Cpp),
            variadic: true,
        };
        let MacroDef::Function {
            params,
            replacement,
            variadic,
        } = def.relexed(Language::C)
        else {
            panic!("function-like shape lost");
        };
        assert_eq!(params, vec!["x".to_string(), "__VA_ARGS__".to_string()]);
        assert!(variadic);
        let k: Vec<TokenKind> = replacement.iter().map(|t| t.kind.clone()).collect();
        assert_eq!(k[0], TokenKind::Hash);
        assert_eq!(k[1], TokenKind::Identifier("x".into()));
        assert_eq!(k[2], TokenKind::Punct("##"));
        assert_eq!(
            &k[k.len() - 2..],
            &[
                TokenKind::Identifier("R".into()),
                TokenKind::String("\"(x)\"".into())
            ]
        );
    }
}
