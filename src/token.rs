use crate::source_map::Span;

/// The lexical category of a `Token`.
/// 
/// Note that we don't have categories for whitespace or comments. Those are
/// discarded immediately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Ident,

    // Literals
    IntLit,    

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
    Comma,
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

/// The smallest lexical unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token {
    /// The type of the token.
    ty: TokenType,

    /// The span of text the token came from.
    span: Span,
}

impl Token {
    /// Create a new `Token` from it's category and type.
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self { ty, span }
    }

    /// Get the `TokenType` of the `Token`.
    pub fn ty(&self) -> TokenType {
        self.ty
    }

    /// Returns the `Span` the `Token` covers.
    pub fn span(&self) -> Span {
        self.span
    }
}
