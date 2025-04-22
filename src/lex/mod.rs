//! The lexer converts a `Source` into a series of `Token`s.

use crate::{
    diagnostics::{specifics::lex::mismatched_close_paren, Diagnostic},
    source_map::{Source, Span},
    FResult,
};
use token::{TokenErrorTy, TokenTree, TokenType};

pub mod token;

pub fn lex_source(source: &Source) -> FResult<Vec<TokenTree>> {
    let tokens = Lexer::new(source).get_tokens();

    let mut errors = Vec::new();
    find_errors(&tokens, &source, &mut errors);

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a Source) -> Self {
        let cursor = Cursor::new(source);
        Self { cursor }
    }

    fn get_tokens(&mut self) -> Vec<TokenTree> {
        let mut paren_stack: Vec<(TokenType, Span, Vec<TokenTree>)> = Vec::new();
        let mut tokens = Vec::new();

        while let Some(next) = self.cursor.pop() {
            match next {
                '(' | '{' | '[' => {
                    let ty = TokenType::new_from_paren(next);
                    paren_stack.push((ty, self.cursor.popped_as_span(), tokens));
                    tokens = Vec::new();
                }
                ')' | '}' | ']' => {
                    let Some((open_ty, open_span, previous_tokens)) = paren_stack.pop() else {
                        // There is no matching parenthesis for this one so
                        // just replace this token with an error.
                        let token_ty = TokenType::Error(TokenErrorTy::UnmatchedCloseParen);
                        tokens.push(self.cursor.popped_as_token(token_ty));
                        continue;
                    };

                    let close_ty = TokenType::new_from_paren(next);
                    let close_span = self.cursor.popped_as_span();

                    if open_ty != close_ty {
                        // If the types don't match we will still build the tree
                        // but we will also add an extra error token at the end
                        // of the children.
                        let token = TokenTree::new_error(
                            TokenErrorTy::MismatchedParenTy { open_span },
                            close_span,
                        );
                        tokens.push(token);
                    }

                    let whole_span = Span::union(open_span, close_span);
                    let tree = TokenTree::new_nested(open_ty, whole_span, tokens);

                    // Restore the previous tokens and add the tree we
                    // just built at the end.
                    tokens = previous_tokens;
                    tokens.push(tree);
                }
                _ => {
                    let Some(ty) = self.next_leaf_ty(next) else {
                        // Comments, whitespace, etc. get ignored.
                        continue;
                    };
                    tokens.push(self.cursor.popped_as_token(ty));
                }
            }
        }

        // If there is still anything in the stack then that means we had
        // unmatched opening parenthesis. We will just ignore those opening
        // parenthesis by replacing them with an error token and concatenating
        // the whole stack into the current tokens vec.
        for (_, open_span, mut prev_tokens) in paren_stack.into_iter().rev() {
            let err_token = TokenTree::new_error(TokenErrorTy::UnmatchedOpenParen, open_span);

            prev_tokens.push(err_token);
            prev_tokens.extend(tokens);
            tokens = prev_tokens;
        }

        tokens
    }

    fn next_leaf_ty(&mut self, next: char) -> Option<TokenType> {
        let cursor = &mut self.cursor;

        let ty = match next {
            // Whitespace
            _ if next.is_ascii_whitespace() => {
                cursor.ignore();
                return None;
            }

            // Comments
            '/' if cursor.peek_is('/') => {
                while !cursor.peek_is('\n') {
                    cursor.pop();
                }
                cursor.ignore();
                return None;
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

                ident_token_ty(cursor.popped_text())
            }

            // Symbols
            '+' => TokenType::Plus,
            '-' if cursor.peek_is('>') => {
                cursor.pop();
                TokenType::RArrow
            }
            '-' => TokenType::Minus,
            '*' => TokenType::Mul,
            '/' => TokenType::Div,
            '!' if cursor.peek_is('=') => {
                cursor.pop();
                TokenType::NotEq
            }
            '!' => TokenType::Not,
            '|' if cursor.peek_is('|') => {
                cursor.pop();
                TokenType::OrOr
            }
            '&' if cursor.peek_is('&') => {
                cursor.pop();
                TokenType::AndAnd
            }
            '=' if cursor.peek_is('=') => {
                cursor.pop();
                TokenType::EqEq
            }
            '=' => TokenType::Eq,
            '<' if cursor.peek_is('=') => {
                cursor.pop();
                TokenType::Lte
            }
            '<' => TokenType::Lt,
            '>' if cursor.peek_is('=') => {
                cursor.pop();
                TokenType::Gte
            }
            '>' => TokenType::Gt,
            ';' => TokenType::Semicolon,
            ':' => TokenType::Colon,
            ',' => TokenType::Comma,

            // Parenthesis
            '(' | ')' | '{' | '}' | '[' | ']' => {
                // Parenthesis indicate this is not a leaf so we should not
                // see these.
                unreachable!();
            }

            // Unrecognized character is an error.
            _ => TokenType::Error(TokenErrorTy::IllegalChar),
        };

        Some(ty)
    }
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
    source: &'a Source,
    byte_offset: usize,
    span_offset: usize,
    span_len: usize,
}

impl<'a> Cursor<'a> {
    fn new(source: &'a Source) -> Self {
        Self {
            source: source,
            byte_offset: 0,
            span_offset: 0,
            span_len: 0,
        }
    }

    fn text(&self) -> &str {
        self.source.text()
    }

    fn remaining_text(&self) -> &str {
        &self.text()[self.byte_offset..]
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

    fn popped_text(&self) -> &str {
        &self.text()[self.span_offset..self.span_offset + self.span_len]
    }

    fn popped_as_span(&mut self) -> Span {
        let span = self.source.span_with_len(self.span_offset, self.span_len);

        self.span_offset = self.byte_offset;
        self.span_len = 0;

        span
    }

    fn popped_as_token(&mut self, ty: TokenType) -> TokenTree {
        TokenTree::new(ty, self.popped_as_span())
    }

    fn ignore(&mut self) {
        self.span_offset = self.byte_offset;
        self.span_len = 0;
    }
}

fn find_errors(tokens: &[TokenTree], source: &Source, errors: &mut Vec<Diagnostic>) {
    for token in tokens {
        find_errors(token.children(), source, errors);

        let TokenType::Error(ty) = token.ty() else {
            continue;
        };

        use crate::diagnostics::specifics::lex;
        use TokenErrorTy as TET;

        let error = match ty {
            TET::IllegalChar => lex::illegal_char(token.span(), source),
            TET::UnmatchedOpenParen => lex::unmatched_open_paren(token.span(), source),
            TET::UnmatchedCloseParen => lex::unmatched_close_paren(token.span(), source),
            TET::MismatchedParenTy { open_span } => {
                mismatched_close_paren(open_span, token.span(), source)
            }
        };
        errors.push(error);
    }
}
