use std::collections::HashSet;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Identifier(String),
    Number(String),
    String(String),
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
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            col: 1,
        }
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
            return self.read_string(line, col);
        }

        if ch == '\'' {
            return self.read_char(line, col);
        }

        if ch.is_ascii_digit() {
            return self.read_number(line, col);
        }

        if is_ident_start(ch) {
            return self.read_identifier(line, col);
        }

        if "+-<>=!&|^~*/%.,;:()[]{}?\\".contains(ch) {
            let mut s = String::new();
            s.push(ch);
            self.advance_char();
            // two-char operators
            if self.pos < self.input.len() {
                let two = format!("{}{}", s, self.peek_char());
                if matches!(
                    two.as_str(),
                    "<<" | ">>"
                        | "<="
                        | ">="
                        | "=="
                        | "!="
                        | "&&"
                        | "||"
                        | "++"
                        | "--"
                        | "+="
                        | "-="
                        | "*="
                        | "/="
                        | "%="
                        | "&="
                        | "|="
                        | "^="
                        | "->"
                ) {
                    s = two;
                    self.advance_char();
                }
            }
            return Token::new(TokenKind::Punct(s), line, col);
        }

        // Unknown char - skip
        self.advance_char();
        self.next_token()
    }

    fn read_string(&mut self, line: u32, col: u32) -> Token {
        let mut s = String::new();
        self.advance_char(); // opening "
        while !self.is_at_end() && self.peek_char() != '"' {
            if self.peek_char() == '\\' {
                s.push('\\');
                self.advance_char();
                if !self.is_at_end() {
                    s.push(self.peek_char());
                    self.advance_char();
                }
            } else if self.peek_char() == '\n' {
                break;
            } else {
                s.push(self.peek_char());
                self.advance_char();
            }
        }
        if !self.is_at_end() && self.peek_char() == '"' {
            self.advance_char();
        }
        Token::new(TokenKind::String(s), line, col)
    }

    fn read_char(&mut self, line: u32, col: u32) -> Token {
        let mut s = String::new();
        self.advance_char();
        while !self.is_at_end() && self.peek_char() != '\'' {
            if self.peek_char() == '\\' {
                s.push('\\');
                self.advance_char();
                if !self.is_at_end() {
                    s.push(self.peek_char());
                    self.advance_char();
                }
            } else {
                s.push(self.peek_char());
                self.advance_char();
            }
        }
        if !self.is_at_end() {
            self.advance_char();
        }
        Token::new(TokenKind::Char(s), line, col)
    }

    fn read_number(&mut self, line: u32, col: u32) -> Token {
        let mut s = String::new();
        while !self.is_at_end() {
            let ch = self.peek_char();
            if ch.is_ascii_alphanumeric()
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

fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
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
        let tokens = Lexer::new("int x = 42;").tokenize();
        assert!(tokens
            .iter()
            .any(|t| matches!(&t.kind, TokenKind::Identifier(s) if s == "int")));
        assert!(tokens
            .iter()
            .any(|t| matches!(&t.kind, TokenKind::Number(s) if s == "42")));
    }
}
