use crate::source_map::Span;

pub enum TokenTree {
    Leaf(Token),
    Node(TokenTreeNode),
}

pub struct TokenTreeNode {
    pub paren_ty: ParenType,
    pub left: Span,
    pub right: Span,
    pub children: Vec<TokenTree>,
}

impl TokenTreeNode {
    pub fn new(ty: ParenType, left: Span, right: Span, children: Vec<TokenTree>) -> Self {
        Self { paren_ty: ty, left, right, children }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParenType {
    Paren,
    Bracket,
    Square,
}

impl ParenType {
    pub fn new_from_char(char: char) -> ParenType {
        match char {
            '(' | ')' => ParenType::Paren,
            '{' | '}' => ParenType::Bracket,
            '[' | ']' => ParenType::Square,
            _ => unreachable!("Only call new_from_char on parenthesis.")
        }
    }
}

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
    Error(TokenErrorTy),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenErrorTy {
    IllegalChar,
    UnmatchedOpenParen,
    UnmatchedCloseParen,
    MismatchedParenTy(ParenType),
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
