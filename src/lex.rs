use crate::{
    source_map::{SourceId, SourceMap},
    span::Span,
    token::{Token, TokenType},
};

pub fn lex_source(sm: &SourceMap, src_id: SourceId) -> Vec<Token> {
    let mut cursor = Cursor::new(sm, src_id);
    let mut tokens = Vec::new();

    while let Some(next) = cursor.pop() {
        let token_ty = match next {
            // Whitespace
            _ if next.is_ascii_whitespace() => {
                cursor.ignore();
                continue;
            }

            // Comments
            '/' if cursor.peek_is('/') => {
                while !cursor.peek_is('\n') {
                    cursor.pop();
                }
                cursor.ignore();
                continue;
            }

            // Literals
            _ if next.is_ascii_digit() => {
                while cursor.peek().is_some_and(|c| c.is_ascii_digit()) {
                    cursor.pop();
                }
                TokenType::IntLit
            }

            // Identifiers and keywords
            _ if char_can_start_ident(next) => {
                while cursor.peek().is_some_and(char_can_continue_ident) {
                    cursor.pop();
                }

                ident_token_ty(cursor.pop_text())
            }

            // Symbols
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '{' => TokenType::LBracket,
            '}' => TokenType::RBracket,
            '[' => TokenType::LSquare,
            ']' => TokenType::RSquare,
            '+' => TokenType::Plus,
            '-' if cursor.peek_is('>') => TokenType::RArrow,
            '-' => TokenType::Minus,
            '*' => TokenType::Mul,
            '/' => TokenType::Div,
            '!' if cursor.peek_is('=') => TokenType::NotEq,
            '!' => TokenType::Not,
            '|' if cursor.peek_is('|') => TokenType::OrOr,
            '&' if cursor.peek_is('&') => TokenType::AndAnd,
            '=' if cursor.peek_is('=') => TokenType::EqEq,
            '=' => TokenType::Eq,
            '<' if cursor.peek_is('=') => TokenType::Lte,
            '<' => TokenType::Lt,
            '>' if cursor.peek_is('=') => TokenType::Gte,
            '>' => TokenType::Gt,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,

            // Unrecognized character is an error.
            _ => TokenType::Error,
        };

        let token = cursor.pop_as_token(token_ty);
        tokens.push(token);
    }

    tokens.push(cursor.pop_as_token(TokenType::EOF));

    tokens
}

fn char_can_continue_ident(c: char) -> bool {
    char_can_start_ident(c) || c.is_ascii_digit()
}

fn char_can_start_ident(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn ident_token_ty(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Fn,
        "let" => TokenType::Let,
        "if" => TokenType::If,
        "while" => TokenType::While,
        "for" => TokenType::For,
        _ => TokenType::Ident,
    }
}

struct Cursor<'a> {
    src_id: SourceId,
    text: &'a str,

    byte_offset: usize,
    span_offset: usize,
    span_len: usize,
}

impl<'a> Cursor<'a> {
    fn new(sm: &'a SourceMap<'a>, src_id: SourceId) -> Self {
        Self {
            src_id,
            text: sm.get_source(src_id).text,
            byte_offset: 0,
            span_offset: 0,
            span_len: 0,
        }
    }

    fn remaining_text(&self) -> &str {
        &self.text[self.byte_offset..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining_text().chars().next()
    }

    fn peek_is(&self, c: char) -> bool {
        self.peek() == Some(c)
    }

    fn pop(&mut self) -> Option<char> {
        let c = self.peek()?;

        self.span_len += c.len_utf8();
        self.byte_offset += c.len_utf8();

        Some(c)
    }

    fn pop_text(&self) -> &str {
        &self.text[self.span_offset..self.span_offset + self.span_len]
    }

    fn pop_as_span(&mut self) -> Span {
        let span = Span {
            src_id: self.src_id,
            byte_offset: self.span_offset,
            byte_len: self.span_len,
        };

        self.span_offset = self.span_len;
        self.span_len = 0;

        span
    }

    fn pop_as_token(&mut self, ty: TokenType) -> Token {
        Token::new(ty, self.pop_as_span())
    }

    fn ignore(&mut self) {
        self.span_offset = self.span_len;
        self.span_len = 0;
    }
}
