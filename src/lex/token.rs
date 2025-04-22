use crate::source_map::Span;

pub struct TokenTree {
    ty: TokenType,
    span: Span,
    children: Vec<TokenTree>,
}

impl TokenTree {
    pub fn new(ty: TokenType, span: Span) -> Self {
        Self {
            ty,
            span,
            children: Vec::new(),
        }
    }

    pub fn new_error(error_ty: TokenErrorTy, span: Span) -> Self {
        Self::new(TokenType::Error(error_ty), span)
    }

    pub fn new_nested(ty: TokenType, span: Span, children: Vec<TokenTree>) -> Self {
        assert!(ty.is_nested(), "Only nested tokens can have children");

        Self { ty, span, children }
    }

    pub fn ty(&self) -> TokenType {
        self.ty
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn children(&self) -> &[TokenTree] {
        &self.children
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

    // Nested
    Parens,
    Brackets,
    CurlyBrackets,

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

    // Error
    Error(TokenErrorTy),
}

impl TokenType {
    pub fn new_from_paren(char: char) -> Self {
        match char {
            '(' | ')' => Self::Parens,
            '[' | ']' => Self::Brackets,
            '{' | '}' => Self::CurlyBrackets,
            _ => unreachable!("Only call new_from_char on parenthesis."),
        }
    }

    pub fn is_nested(&self) -> bool {
        matches!(
            self,
            TokenType::Parens | TokenType::Brackets | TokenType::CurlyBrackets
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenErrorTy {
    IllegalChar,
    UnmatchedOpenParen,
    UnmatchedCloseParen,
    MismatchedParenTy { open_span: Span },
}
