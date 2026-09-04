use crate::Language;
use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier(String),
    Number(String),
    /// A string literal spelled exactly as written — encoding prefix,
    /// quotes and, for a C++11 raw string, delimiters and embedded newlines
    /// (`"a"`, `L"a"`, `u8R"~(a "b")~"`). Carrying the spelling means the
    /// literal is re-emitted verbatim; the few consumers that need the body
    /// (`#include "…"`, `#if 'c'`) strip the delimiters themselves.
    String(String),
    /// A character literal spelled as written, prefix and quotes included
    /// (`'a'`, `L'\n'`).
    Char(String),
    Punct(String),
    Hash, // #
    Newline,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: u32,
    pub col: u32,
    /// Macros that must not expand this token again (C11 6.10.3.4 hide set).
    pub(crate) hidden: Option<Arc<HashSet<String>>>,
    /// Set on a macro-argument token that a `\`-newline splice, and nothing
    /// else, separates from the token before it. Phase 2 deletes the splice
    /// before tokenizing (C11 5.1.1.2p1), so the two are adjacent in the
    /// spliced source even though their `line`/`col` say otherwise; `#`
    /// stringizing has to spell them with no space between.
    pub(crate) spliced_before: bool,
    /// For a token that came out of a macro replacement list: the
    /// `(line, col)` of the outermost invocation that produced it, in the
    /// file being processed. `line`/`col` keep the definition-site
    /// coordinates (they still decide whitespace adjacency); this is what
    /// the LineMap and `__LINE__` report, so macro-expanded code attributes
    /// to its expansion site even through forwarding macros.
    pub(crate) origin: Option<(u32, u32)>,
}

impl Token {
    pub fn new(kind: TokenKind, line: u32, col: u32) -> Self {
        Self {
            kind,
            line,
            col,
            hidden: None,
            origin: None,
            spliced_before: false,
        }
    }

    /// Where this token attributes to: its own position for source text,
    /// the outermost invocation for macro-expanded text.
    pub(crate) fn expansion_site(&self) -> (u32, u32) {
        self.origin.unwrap_or((self.line, self.col))
    }

    pub(crate) fn is_hidden(&self, name: &str) -> bool {
        self.hidden.as_ref().is_some_and(|h| h.contains(name))
    }

    /// Paint this replacement-list token with the invoking token's hide set
    /// plus `name` so the macro is not re-expanded (C11 6.10.3.4), and with
    /// the invocation's expansion site. `origin` may itself be a painted
    /// token (a forwarding macro's body), so its own site is inherited
    /// rather than its definition coordinates.
    pub(crate) fn with_macro_hide(&self, origin: &Token, name: &str) -> Token {
        let mut set = HashSet::new();
        if let Some(h) = &origin.hidden {
            set.extend(h.iter().cloned());
        }
        if let Some(h) = &self.hidden {
            set.extend(h.iter().cloned());
        }
        set.insert(name.to_string());
        Token {
            kind: self.kind.clone(),
            line: self.line,
            col: self.col,
            hidden: Some(Arc::new(set)),
            origin: Some(origin.expansion_site()),
            spliced_before: self.spliced_before,
        }
    }

    pub(crate) fn union_hidden(left: &Token, right: &Token) -> Option<Arc<HashSet<String>>> {
        match (&left.hidden, &right.hidden) {
            (None, None) => None,
            (Some(x), None) | (None, Some(x)) => Some(Arc::clone(x)),
            (Some(x), Some(y)) => {
                let mut s = (**x).clone();
                s.extend(y.iter().cloned());
                Some(Arc::new(s))
            }
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: u32,
    col: u32,
    /// Decides the C++-only token shapes: raw string literals and
    /// user-defined-literal suffixes are one token in C++ and identifier +
    /// literal (or literal + identifier) in C, where the identifier can be
    /// a macro that must still expand.
    language: Language,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, language: Language) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            col: 1,
            language,
        }
    }

    fn is_cpp(&self) -> bool {
        self.language == Language::Cpp
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let is_eof = matches!(tok.kind, TokenKind::Eof);
            tokens.push(tok);
            if is_eof {
                break;
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace_and_comments();
        let line = self.line;
        let col = self.col;

        if self.is_at_end() {
            return Token::new(TokenKind::Eof, line, col);
        }

        let ch = self.peek_char();

        if ch == '\n' {
            self.advance_char();
            return Token::new(TokenKind::Newline, line, col);
        }

        if ch == '#' {
            if self.peek_char_at(1) == '#' {
                self.advance_char();
                self.advance_char();
                return Token::new(TokenKind::Punct("##".to_string()), line, col);
            }
            self.advance_char();
            return Token::new(TokenKind::Hash, line, col);
        }

        if ch == '"' {
            return self.read_string(self.pos, line, col);
        }

        if ch == '\'' {
            return self.read_char(self.pos, line, col);
        }

        if ch.is_ascii_digit() {
            return self.read_number(line, col);
        }

        if is_ident_start(ch) {
            // Only `R`, `u`, `U` and `L` can prefix a literal; every other
            // identifier skips the probe (this is the lexer's hottest path).
            if matches!(ch, 'R' | 'u' | 'U' | 'L') {
                if let Some(tok) = self.read_prefixed_literal(line, col) {
                    return tok;
                }
            }
            return self.read_identifier(line, col);
        }

        if let Some(one) = single_char_punct(ch) {
            self.advance_char();
            // Longest match wins, and every spelling is a `&'static str`, so
            // a punctuator costs one allocation instead of an intermediate
            // `String` per candidate length. `next` is read once and reused.
            let next = self.peek_char();
            let spelling = if ch == '.' && next == '.' && self.peek_char_at(1) == '.' {
                // `...` is the only three-character punctuator the token
                // stream needs; the others in the C++ set are still one token
                // per character (issue #28, docs/PREPROCESSOR.md).
                "..."
            } else {
                two_char_punct(ch, next).unwrap_or(one)
            };
            // `ch` is consumed; take the rest. Punctuators are all ASCII, so
            // the byte length is the character count.
            for _ in 1..spelling.len() {
                self.advance_char();
            }
            return Token::new(TokenKind::Punct(spelling.to_string()), line, col);
        }

        // Unknown char - skip
        self.advance_char();
        self.next_token()
    }

    /// An ordinary string literal whose opening quote is at the current
    /// position; `start` is where the token began (before any encoding
    /// prefix). Escapes are kept as written. A literal cut off by a newline
    /// or end of input gets its closing quote back so the output stays
    /// well-formed.
    fn read_string(&mut self, start: usize, line: u32, col: u32) -> Token {
        self.advance_char(); // opening "
        while !self.is_at_end() && self.peek_char() != '"' {
            if self.peek_char() == '\\' {
                self.advance_char();
                if !self.is_at_end() {
                    self.advance_char();
                }
            } else if self.peek_char() == '\n' {
                break;
            } else {
                self.advance_char();
            }
        }
        let mut spelling = self.close_literal(start, '"');
        self.read_ud_suffix(&mut spelling);
        Token::new(TokenKind::String(spelling), line, col)
    }

    fn read_char(&mut self, start: usize, line: u32, col: u32) -> Token {
        self.advance_char(); // opening '
        while !self.is_at_end() && self.peek_char() != '\'' {
            if self.peek_char() == '\\' {
                self.advance_char();
                if !self.is_at_end() {
                    self.advance_char();
                }
            } else {
                self.advance_char();
            }
        }
        let mut spelling = self.close_literal(start, '\'');
        self.read_ud_suffix(&mut spelling);
        Token::new(TokenKind::Char(spelling), line, col)
    }

    /// Append a user-defined-literal suffix (C++11 [lex.ext]): an identifier
    /// glued directly to the closing quote, as in `"x"_json` or `'c'_w`,
    /// is part of the literal token. Emitting it as a separate Identifier
    /// would put a space before it and change the program's meaning. An
    /// unterminated literal ends at a newline or end of input, so this
    /// never takes anything from the following line. C has no such suffix:
    /// `'a'C` is the literal followed by the identifier `C`, which may be
    /// a macro, so the C lexer leaves the identifier alone.
    fn read_ud_suffix(&mut self, spelling: &mut String) {
        if !self.is_cpp() || self.is_at_end() || !is_ident_start(self.peek_char()) {
            return;
        }
        while !self.is_at_end() && is_ident_continue(self.peek_char()) {
            spelling.push(self.peek_char());
            self.advance_char();
        }
    }

    /// Consume the closing `quote` if it is there and return the literal's
    /// spelling from `start`, with the quote appended when it was missing.
    fn close_literal(&mut self, start: usize, quote: char) -> String {
        let closed = !self.is_at_end() && self.peek_char() == quote;
        if closed {
            self.advance_char();
        }
        let mut spelling = self.input[start..self.pos].to_string();
        if !closed {
            spelling.push(quote);
        }
        spelling
    }

    /// A literal introduced by an identifier character: an encoding prefix
    /// (`u8`, `u`, `U`, `L`) on a string or character literal, or, in C++,
    /// a raw string with or without one (C++11 [lex.string]). Returns
    /// `None` and consumes nothing when the text is an ordinary identifier,
    /// so `Rect`, `u8x`, `L` or `L "x"` (with a space) all lex as before.
    /// C has no raw strings: there `R"(x)"` is the identifier `R` (possibly
    /// a macro) followed by an ordinary string.
    ///
    /// Known limitation: the lexer does no translation-phase-2 splicing, so
    /// a prefix broken by `\`-newline (`R\` newline `"(x)"`, legal C++) is
    /// not recognized here, just as `in\` newline `t` is not one
    /// identifier. Splices are kept as `\` + newline tokens throughout.
    fn read_prefixed_literal(&mut self, line: u32, col: u32) -> Option<Token> {
        let rest = &self.input[self.pos..];
        let enc = ["u8", "u", "U", "L"]
            .iter()
            .find(|p| rest.starts_with(*p))
            .map_or(0, |p| p.len());
        let after = &rest[enc..];
        if self.is_cpp() && after.starts_with("R\"") {
            return self.read_raw_string(enc + 1, line, col);
        }
        if enc == 0 {
            return None;
        }
        let start = self.pos;
        let quote = after.chars().next()?;
        if quote != '"' && quote != '\'' {
            return None;
        }
        for _ in 0..enc {
            self.advance_char();
        }
        Some(if quote == '"' {
            self.read_string(start, line, col)
        } else {
            self.read_char(start, line, col)
        })
    }

    /// Lex a raw string literal whose `"` sits `quote_at` bytes past the
    /// current position (after the encoding prefix and `R`): a
    /// d-char-sequence of at most 16 characters, `(`, an arbitrary body and
    /// `)` + the same d-char-sequence + `"`. Returns `None` and consumes
    /// nothing for a delimiter containing space, `\`, `)` or a control
    /// character, or a literal with no matching closer before end of input,
    /// leaving the text to the identifier/string paths so a malformed
    /// literal costs a couple of bad tokens instead of swallowing the file.
    fn read_raw_string(&mut self, quote_at: usize, line: u32, col: u32) -> Option<Token> {
        const MAX_DELIM: usize = 16;
        let rest = &self.input[self.pos..];
        let after_quote = &rest[quote_at + 1..];
        // The delimiter is at most 16 d-chars, so look for `(` only that
        // far: an `R"..."` that is really a prefixed ordinary string must
        // not scan ahead to some unrelated `(` further down the file.
        let delim_len = after_quote
            .bytes()
            .take(MAX_DELIM + 1)
            .position(|b| b == b'(')?;
        let delim = &after_quote[..delim_len];
        if !delim.bytes().all(is_d_char) {
            return None;
        }
        let body_start = quote_at + 1 + delim_len + 1;
        let closer = format!("){delim}\"");
        let body_len = rest[body_start..].find(&closer)?;
        let total = body_start + body_len + closer.len();
        let mut spelling = rest[..total].to_string();
        let end = self.pos + total;
        while self.pos < end {
            self.advance_char();
        }
        self.read_ud_suffix(&mut spelling);
        Some(Token::new(TokenKind::String(spelling), line, col))
    }

    fn read_number(&mut self, line: u32, col: u32) -> Token {
        let mut s = String::new();
        while !self.is_at_end() {
            let ch = self.peek_char();
            // `_` keeps a ud-suffix such as `10_km` inside the token.
            if ch.is_ascii_alphanumeric()
                || ch == '_'
                || ch == '.'
                || ch == 'x'
                || ch == 'X'
                || ch == 'u'
                || ch == 'U'
                || ch == 'l'
                || ch == 'L'
            {
                s.push(ch);
                self.advance_char();
            } else if ch == '\'' && self.peek_char_at(1).is_ascii_alphanumeric() {
                // C++14 digit separator (1'000'000): skip it so the whole
                // literal stays one Number token.
                self.advance_char();
            } else {
                break;
            }
        }
        Token::new(TokenKind::Number(s), line, col)
    }

    fn read_identifier(&mut self, line: u32, col: u32) -> Token {
        let mut s = String::new();
        while !self.is_at_end() {
            let ch = self.peek_char();
            if is_ident_continue(ch) {
                s.push(ch);
                self.advance_char();
            } else {
                break;
            }
        }
        Token::new(TokenKind::Identifier(s), line, col)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.is_at_end() {
                return;
            }
            let ch = self.peek_char();
            if ch == ' ' || ch == '\t' || ch == '\r' {
                self.advance_char();
                continue;
            }
            if ch == '/' && self.peek_char_at(1) == '/' {
                while !self.is_at_end() && self.peek_char() != '\n' {
                    self.advance_char();
                }
                continue;
            }
            if ch == '/' && self.peek_char_at(1) == '*' {
                self.advance_char();
                self.advance_char();
                while !self.is_at_end() {
                    if self.peek_char() == '*' && self.peek_char_at(1) == '/' {
                        self.advance_char();
                        self.advance_char();
                        break;
                    }
                    self.advance_char();
                }
                continue;
            }
            break;
        }
    }

    fn peek_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap_or('\0')
    }

    fn peek_char_at(&self, offset: usize) -> char {
        self.input[self.pos..].chars().nth(offset).unwrap_or('\0')
    }

    fn advance_char(&mut self) {
        if self.is_at_end() {
            return;
        }
        let ch = self.peek_char();
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.input.len()
    }
}

/// The punctuator spelling for a character that can begin one, or `None` if
/// it cannot. Doubles as the "is this a punctuator?" test, so the dispatch
/// does not also scan a string of the alphabet.
fn single_char_punct(ch: char) -> Option<&'static str> {
    Some(match ch {
        '+' => "+",
        '-' => "-",
        '<' => "<",
        '>' => ">",
        '=' => "=",
        '!' => "!",
        '&' => "&",
        '|' => "|",
        '^' => "^",
        '~' => "~",
        '*' => "*",
        '/' => "/",
        '%' => "%",
        '.' => ".",
        ',' => ",",
        ';' => ";",
        ':' => ":",
        '(' => "(",
        ')' => ")",
        '[' => "[",
        ']' => "]",
        '{' => "{",
        '}' => "}",
        '?' => "?",
        '\\' => "\\",
        _ => return None,
    })
}

/// The two-character operator `a`+`b` spell, or `None` if they do not form
/// one. `b` is `\0` at end of input, which matches nothing.
fn two_char_punct(a: char, b: char) -> Option<&'static str> {
    Some(match (a, b) {
        ('<', '<') => "<<",
        ('>', '>') => ">>",
        ('<', '=') => "<=",
        ('>', '=') => ">=",
        ('=', '=') => "==",
        ('!', '=') => "!=",
        ('&', '&') => "&&",
        ('|', '|') => "||",
        ('+', '+') => "++",
        ('-', '-') => "--",
        ('+', '=') => "+=",
        ('-', '=') => "-=",
        ('*', '=') => "*=",
        ('/', '=') => "/=",
        ('%', '=') => "%=",
        ('&', '=') => "&=",
        ('|', '=') => "|=",
        ('^', '=') => "^=",
        ('-', '>') => "->",
        _ => return None,
    })
}

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

/// Member of a raw string's d-char-sequence: any basic source character
/// except space, parentheses, backslash and control characters.
fn is_d_char(b: u8) -> bool {
    b.is_ascii_graphic() && !matches!(b, b'(' | b')' | b'\\')
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Identifier(s) => write!(f, "id({s})"),
            TokenKind::Number(s) => write!(f, "num({s})"),
            TokenKind::String(s) => write!(f, "str({s})"),
            TokenKind::Char(s) => write!(f, "char({s})"),
            TokenKind::Punct(s) => write!(f, "{s}"),
            TokenKind::Hash => write!(f, "#"),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexes_simple_c() {
        let tokens = Lexer::new("int x = 42;", Language::C).tokenize();
        assert!(tokens
            .iter()
            .any(|t| matches!(&t.kind, TokenKind::Identifier(s) if s == "int")));
        assert!(tokens
            .iter()
            .any(|t| matches!(&t.kind, TokenKind::Number(s) if s == "42")));
    }

    fn kinds_in(src: &str, language: Language) -> Vec<TokenKind> {
        Lexer::new(src, language)
            .tokenize()
            .into_iter()
            .map(|t| t.kind)
            .filter(|k| !matches!(k, TokenKind::Eof))
            .collect()
    }

    /// Token kinds under the C++ lexer (raw strings and ud-suffixes on).
    fn kinds(src: &str) -> Vec<TokenKind> {
        kinds_in(src, Language::Cpp)
    }

    fn kinds_c(src: &str) -> Vec<TokenKind> {
        kinds_in(src, Language::C)
    }

    /// A literal spelled as written (raw or prefixed).
    fn raw(s: &str) -> TokenKind {
        TokenKind::String(s.to_string())
    }

    fn id(s: &str) -> TokenKind {
        TokenKind::Identifier(s.to_string())
    }

    /// An ordinary string literal with the given body.
    fn string(body: &str) -> TokenKind {
        TokenKind::String(format!("\"{body}\""))
    }

    fn chr(s: &str) -> TokenKind {
        TokenKind::Char(s.to_string())
    }

    fn punct(s: &str) -> TokenKind {
        TokenKind::Punct(s.to_string())
    }

    #[test]
    fn raw_string_is_one_token_with_inner_quotes_and_parens() {
        // Issue #14: `R"(a "quoted" b)"` used to lex as `R "(a " quoted " b)"`.
        assert_eq!(
            kinds(r#"const char* j = R"(a "quoted" (b))";"#),
            vec![
                id("const"),
                id("char"),
                punct("*"),
                id("j"),
                punct("="),
                raw(r#"R"(a "quoted" (b))""#),
                punct(";"),
            ]
        );
    }

    #[test]
    fn raw_string_honours_d_char_sequence_delimiter() {
        // `)"` inside the body does not end a `~`-delimited literal.
        assert_eq!(
            kinds(r#"R"~({"k":")~" + v + R"~(",)~""#),
            vec![
                raw(r#"R"~({"k":")~""#),
                punct("+"),
                id("v"),
                punct("+"),
                raw(r#"R"~(",)~""#),
            ]
        );
        assert_eq!(
            kinds(r#"R"~(=((".*?")|(\S*)))~""#),
            vec![raw(r#"R"~(=((".*?")|(\S*)))~""#)]
        );
    }

    #[test]
    fn raw_string_accepts_encoding_prefixes() {
        for prefix in ["u8R", "uR", "UR", "LR"] {
            let src = format!("x = {prefix}\"(y)\";");
            assert_eq!(
                kinds(&src),
                vec![
                    id("x"),
                    punct("="),
                    raw(&format!("{prefix}\"(y)\"")),
                    punct(";")
                ],
                "{src}"
            );
        }
    }

    #[test]
    fn raw_string_spans_lines_as_one_token() {
        let src = "a = R\"~({\n  \"k\": 1,\n  \"v\": 2})~\";\nint z;";
        assert_eq!(
            kinds(src),
            vec![
                id("a"),
                punct("="),
                raw("R\"~({\n  \"k\": 1,\n  \"v\": 2})~\""),
                punct(";"),
                TokenKind::Newline,
                id("int"),
                id("z"),
                punct(";"),
            ]
        );
        // The `;` after the literal sits on line 3 right after `)~"`.
        let toks = Lexer::new(src, Language::Cpp).tokenize();
        let semi = &toks[3];
        assert_eq!((semi.line, semi.col), (3, 13), "{semi:?}");
        let int_tok = &toks[5];
        assert_eq!((int_tok.line, int_tok.col), (4, 1), "{int_tok:?}");
    }

    #[test]
    fn user_defined_literal_suffix_stays_in_the_token() {
        // A ud-suffix is part of the literal token (C++11 [lex.ext]); a
        // separate Identifier would gain a space on emission and break
        // `R"(json)"_json` into `R"(json)" _json`.
        assert_eq!(
            kinds(r#"auto j = R"(json)"_json;"#),
            vec![
                id("auto"),
                id("j"),
                punct("="),
                raw(r#"R"(json)"_json"#),
                punct(";")
            ]
        );
        assert_eq!(kinds(r#"u8R"~(x)~"_w"#), vec![raw(r#"u8R"~(x)~"_w"#)]);
        assert_eq!(
            kinds(r#""abc"_json + "s"s"#),
            vec![raw(r#""abc"_json"#), punct("+"), raw(r#""s"s"#)]
        );
        assert_eq!(kinds("L'c'_x"), vec![chr("L'c'_x")]);
        assert_eq!(
            kinds("10_km + 1.5_m"),
            vec![
                TokenKind::Number("10_km".to_string()),
                punct("+"),
                TokenKind::Number("1.5_m".to_string())
            ]
        );
        // Whitespace between the literal and an identifier keeps them apart.
        assert_eq!(
            kinds(r#"R"(x)" _json"#),
            vec![raw(r#"R"(x)""#), id("_json")]
        );
        assert_eq!(kinds(r#""x" s"#), vec![string("x"), id("s")]);
        // An unterminated string takes no suffix from the next line.
        assert_eq!(
            kinds("\"abc\nx"),
            vec![string("abc"), TokenKind::Newline, id("x")]
        );
    }

    #[test]
    fn raw_string_backslashes_and_comment_markers_are_literal() {
        // No escape processing and no comment stripping inside the body.
        assert_eq!(
            kinds(r#"R"(\n // not a comment /* nor this */ \")""#),
            vec![raw(r#"R"(\n // not a comment /* nor this */ \")""#)]
        );
    }

    #[test]
    fn r_not_starting_a_raw_string_stays_an_identifier() {
        // No `(` after the quote: an ordinary (prefixed) string literal.
        assert_eq!(kinds(r#"R"abc""#), vec![id("R"), string("abc")]);
        // `R` is only a prefix when it is the whole identifier before `"`.
        assert_eq!(kinds(r#"FOOR"(x)""#), vec![id("FOOR"), string("(x)")]);
        // A space between `R` and `"` makes it two tokens.
        assert_eq!(kinds(r#"R "(x)""#), vec![id("R"), string("(x)")]);
        // Lower-case `r` is not a raw-string prefix.
        assert_eq!(kinds(r#"r"(x)""#), vec![id("r"), string("(x)")]);
        // A plain identifier that happens to start with R.
        assert_eq!(kinds("Rect r;"), vec![id("Rect"), id("r"), punct(";")]);
    }

    #[test]
    fn malformed_raw_string_falls_back_to_ordinary_lexing() {
        // Delimiter longer than 16 chars is not a d-char-sequence.
        assert_eq!(
            kinds(r#"R"abcdefghijklmnopq(x)abcdefghijklmnopq""#),
            vec![id("R"), string("abcdefghijklmnopq(x)abcdefghijklmnopq")]
        );
        // Space / backslash / `)` are not d-chars.
        assert_eq!(kinds(r#"R"a b(x)a b""#), vec![id("R"), string("a b(x)a b")]);
        // (`\"` is an escape for the ordinary string reader, which then
        // runs to end of input — the pre-existing behaviour for a bad string.)
        assert_eq!(kinds(r#"R"a\(x)a\""#), vec![id("R"), string(r#"a\(x)a\""#)]);
        // No closer with the right delimiter before end of input: fall back
        // instead of swallowing the rest of the file.
        assert_eq!(
            kinds("R\"~(x)\" y\nz"),
            vec![
                id("R"),
                string("~(x)"),
                id("y"),
                TokenKind::Newline,
                id("z")
            ]
        );
    }

    #[test]
    fn encoding_prefixed_literals_are_one_token() {
        for prefix in ["u8", "u", "U", "L"] {
            let src = format!("a = {prefix}\"s\" + {prefix}'c';");
            assert_eq!(
                kinds(&src),
                vec![
                    id("a"),
                    punct("="),
                    raw(&format!("{prefix}\"s\"")),
                    punct("+"),
                    chr(&format!("{prefix}'c'")),
                    punct(";"),
                ],
                "{src}"
            );
        }
        // Escapes inside a prefixed literal are kept as written.
        assert_eq!(
            kinds(r#"L"a\"b" L'\n'"#),
            vec![raw(r#"L"a\"b""#), chr(r#"L'\n'"#)]
        );
    }

    #[test]
    fn prefix_lookalikes_stay_identifiers() {
        assert_eq!(
            kinds("u8x = L;"),
            vec![id("u8x"), punct("="), id("L"), punct(";")]
        );
        assert_eq!(kinds(r#"L "x""#), vec![id("L"), string("x")]);
        assert_eq!(kinds("U + u"), vec![id("U"), punct("+"), id("u")]);
        assert_eq!(kinds("Lx'c'"), vec![id("Lx"), chr("'c'")]);
    }

    #[test]
    fn literals_carry_their_spelling() {
        assert_eq!(
            kinds(r#""a\"b" 'c' '\''"#),
            vec![string(r#"a\"b"#), chr("'c'"), chr(r#"'\''"#)]
        );
        // A string cut off by a newline gets its closing quote back.
        assert_eq!(
            kinds("\"open\nint x;"),
            vec![
                string("open"),
                TokenKind::Newline,
                id("int"),
                id("x"),
                punct(";")
            ]
        );
    }

    #[test]
    fn c_has_no_raw_strings() {
        // In C `R` is an identifier (maybe a macro) and `"(x)"` an ordinary
        // string; the same text is one raw-string token in C++.
        assert_eq!(kinds_c(r#"R"(x)""#), vec![id("R"), string("(x)")]);
        assert_eq!(kinds(r#"R"(x)""#), vec![raw(r#"R"(x)""#)]);
        assert_eq!(
            kinds_c(r#"u8R"~(a "b")~""#),
            vec![id("u8R"), string("~(a "), id("b"), string(")~")]
        );
        // Encoding prefixes exist in C too and stay glued to the literal.
        assert_eq!(
            kinds_c(r#"L"w" u8"s" u'c'"#),
            vec![raw(r#"L"w""#), raw(r#"u8"s""#), chr("u'c'")]
        );
    }

    #[test]
    fn c_has_no_user_defined_literal_suffix() {
        // `'a'C` is the literal followed by the identifier `C` in C; the
        // identifier may be a macro and must stay its own token.
        assert_eq!(kinds_c("'a'C"), vec![chr("'a'"), id("C")]);
        assert_eq!(kinds("'a'C"), vec![chr("'a'C")]);
        assert_eq!(kinds_c(r#""x"_s"#), vec![string("x"), id("_s")]);
        assert_eq!(kinds_c("L'c'_x"), vec![chr("L'c'"), id("_x")]);
        // A pp-number swallows a trailing identifier in both languages
        // (C11 6.4.8): `10_km` is one token either way.
        assert_eq!(
            kinds_c("10_km"),
            vec![TokenKind::Number("10_km".to_string())]
        );
    }

    #[test]
    fn ellipsis_is_one_punctuator() {
        // Issue #28: `...` used to lex as three `.` tokens, which the token
        // re-speller wrote back as `. . .`, breaking every variadic
        // declaration.
        assert_eq!(
            kinds_c("int f(const char *fmt, ...);"),
            vec![
                id("int"),
                id("f"),
                punct("("),
                id("const"),
                id("char"),
                punct("*"),
                id("fmt"),
                punct(","),
                punct("..."),
                punct(")"),
                punct(";"),
            ]
        );
        assert_eq!(
            kinds("template <class... T> void f(T... a);")[3],
            punct("...")
        );
    }

    #[test]
    fn two_dots_stay_separate_tokens() {
        // Only a full `...` is one token; a shorter run is still one `.`
        // each, so `x..y` keeps its token boundaries.
        assert_eq!(
            kinds_c("x..y"),
            vec![id("x"), punct("."), punct("."), id("y")]
        );
        assert_eq!(kinds_c("....."), vec![punct("..."), punct("."), punct(".")]);
    }
}
