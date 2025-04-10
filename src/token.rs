use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Ident,

    // Literals
    NumLit,    
    StringLit,

    // Keywords
    Fn,
    Let,
    If,
    While,
    For,

    // Parenthesis
    LParen,
    RParen,
    LBracket,
    RBracket,
    LSquare,
    RSquare,

    // Symbols
    Semicolon,
    Colon,
    RArrow,

    Plus,
    Minus,
    Mul,
    Div,
    Not,

    OrOr,
    AndAnd,

    Eq,
    EqEq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,

    // Other
    EOF,
    Error
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    ty: TokenType,
    span: Span,
}

impl Token {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self { ty, span }
    }
}
