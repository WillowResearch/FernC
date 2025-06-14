//! Implements a lexer for the following grammar:
//!
//! ```grammar
//! file ::= declaration*
//!
//! declaration ::=
//!     | fn_decl
//!     | struct_decl
//!
//! fn_decl ::= FN IDENT fn_args (R_ARROW type)? block
//! fn_args ::= L_PAREN (fn_arg COMMA)* fn_arg? R_PAREN
//! fn_arg  ::= IDENT COLON type
//!
//! struct_decl ::= TODO
//!
//! block ::= L_CURLY statement* expr? R_CURLY
//! statement ::=
//!     | SEMICOLON
//!     | let_statement
//!     | expr_statement
//!
//! let_statement  ::= LET IDENT (COLON TYPE)? EQUAL expr SEMICOLON
//! expr_statement ::=
//!     | expr_with_block SEMICOLON?
//!     | expr_without_block SEMICOLON
//!
//! expr ::=
//!     | expr_with_block
//!     | expr_without_block
//!
//! expr_with_block ::=
//!     | if_expr
//!
//! expr_without_block ::=
//!     | literal_expr
//!     | ident_expr
//!     | field_access_expr
//!     | fn_call_expr
//!     | operator_expr
//!     | paren_expr
//!
//! if_expr ::= IF expr block (ELSE IF expr block)* (ELSE block)?
//!
//! literal_expr      ::= INT_LITERAL | BOOL_LITERAL
//! ident_expr        ::= IDENT
//! field_access_expr ::= expr DOT IDENT
//! fn_call_expr      ::= expr L_PAREN (expr COMMA)* expr? R_PAREN
//! paren_expr        ::= L_PAREN expr R_PAREN
//! operator_expr     ::= expr OPERATOR expr
//!
//! type ::= IDENT
//! ```
//!

use crate::{
    diagnostics::{self, Diagnostic},
    lex::{
        lex_source,
        token::{TokenTree, TokenType},
    },
    source_map::{Source, Span},
    FResult,
};
use ast::{BlockAst, DeclarationAst, FileAst, FnArgAst, FnDeclAst, FnReturnTypeAst, TypeAst};

pub mod ast;
pub mod visit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SyncStatus {
    Synced,
    Unsynced,
}

type PResult<T> = Result<T, SyncStatus>;

pub fn parse_source(source: &Source) -> FResult<FileAst> {
    let lexed = lex_source(source)?;

    let mut cursor = Cursor::new(&lexed);
    let mut diagnostics = Vec::new();
    let parsed = parse_file(&mut cursor, &mut diagnostics);

    match parsed {
        Ok(file) if diagnostics.is_empty() => Ok(file),
        _ => Err(diagnostics),
    }
}

fn parse_file(cursor: &mut Cursor, diags: &mut Vec<Diagnostic>) -> PResult<FileAst> {
    let mut declarations = Vec::new();

    while !cursor.is_eof() {
        if let Ok(decl) = parse_decl(cursor, diags) {
            declarations.push(decl)
        }
    }

    let file = FileAst { declarations };
    Ok(file)
}

fn parse_decl(cursor: &mut Cursor, diags: &mut Vec<Diagnostic>) -> PResult<DeclarationAst> {
    let decl = match () {
        _ if cursor.peek_is(TokenType::Fn) => parse_fn(cursor, diags).map(DeclarationAst::FnDecl),
        _ => Err(SyncStatus::Unsynced),
    };

    if let Err(SyncStatus::Unsynced) = decl {
        cursor.sync_to(&[TokenType::Fn]);
        Err(SyncStatus::Synced)
    } else {
        decl
    }
}

fn parse_fn(cursor: &mut Cursor, diags: &mut Vec<Diagnostic>) -> PResult<FnDeclAst> {
    fn parse_fn_arg(cursor: &mut Cursor, diags: &mut Vec<Diagnostic>) -> PResult<FnArgAst> {
        let name = cursor.pop_expect(TokenType::Ident)?;
        let colon = cursor.pop_expect(TokenType::Colon)?;
        let ty = parse_ty(cursor, diags)?;

        Ok(FnArgAst {
            name: name.span(),
            colon: colon.span(),
            ty,
        })
    }

    fn parse_fn_args(cursor: &mut Cursor, diags: &mut Vec<Diagnostic>) -> PResult<Vec<FnArgAst>> {
        let args_tokens = cursor.pop_expect(TokenType::Parens)?;
        let mut cursor = Cursor::new(args_tokens.children());

        let mut args = Vec::new();

        while !cursor.is_eof() {
            match parse_fn_arg(&mut cursor, diags) {
                Ok(arg) => args.push(arg),
                Err(SyncStatus::Synced) => {}
                Err(SyncStatus::Unsynced) => cursor.sync_to(&[TokenType::Comma]),
            }

            if cursor.pop_if(TokenType::Comma).is_none() {
                break;
            }
        }

        if !cursor.is_eof() {
            todo!();
        }

        Ok(args)
    }

    fn parse_fn_return_ty(
        cursor: &mut Cursor,
        diags: &mut Vec<Diagnostic>,
    ) -> PResult<Option<FnReturnTypeAst>> {
        let Some(r_arrow) = cursor.pop_if(TokenType::RArrow) else {
            return Ok(None);
        };
        let ty = parse_ty(cursor, diags);

        Ok(Some(FnReturnTypeAst {
            r_arrow: r_arrow.span(),
            ty: ty?,
        }))
    }

    let fn_kew = cursor.pop_assert(TokenType::Fn);
    let name_ident = cursor.pop_expect(TokenType::Ident);
    let args = parse_fn_args(cursor, diags);
    let return_ty = parse_fn_return_ty(cursor, diags);
    let body = parse_block(cursor, diags);

    Ok(FnDeclAst {
        // If we found the body we are probably synchronized.
        body: body?,
        // Otherwise give up.
        fn_kw: fn_kew.span(),
        name_ident: name_ident?.span(),
        args: args?,
        return_ty: return_ty?,
    })
}

fn parse_block(cursor: &mut Cursor, diags: &mut Vec<Diagnostic>) -> PResult<BlockAst> {
    // todo!()
    cursor.pop_expect(TokenType::CurlyBrackets);
    Ok(BlockAst {
        statements: Vec::new(),
        return_expr: None,
    })
}

fn parse_ty(cursor: &mut Cursor, diags: &mut Vec<Diagnostic>) -> PResult<TypeAst> {
    // todo!()
    let name = cursor.pop_expect(TokenType::Ident);
    Ok(TypeAst { name_ident: name?.span() })
}

struct Cursor<'a> {
    tokens: &'a [TokenTree],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(tokens: &'a [TokenTree]) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &'a TokenTree {
        &self.tokens[self.pos]
    }

    fn pop(&mut self) -> &'a TokenTree {
        assert!(!self.is_eof());

        self.pos += 1;
        &self.tokens[self.pos - 1]
    }

    fn peek_is(&self, ty: TokenType) -> bool {
        !self.is_eof() && self.peek().ty() == ty
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn pop_assert(&mut self, ty: TokenType) -> &'a TokenTree {
        assert!(self.peek_is(ty));
        self.pop()
    }

    fn pop_expect(&mut self, ty: TokenType) -> PResult<&'a TokenTree> {
        if self.peek_is(ty) {
            Ok(self.pop())
        } else {
            Err(SyncStatus::Unsynced)
        }
    }

    fn pop_if(&mut self, ty: TokenType) -> Option<&'a TokenTree> {
        self.peek_is(ty).then(|| self.pop())
    }

    fn sync_to(&mut self, sync_tokens: &[TokenType]) {
        while !self.is_eof() && !sync_tokens.contains(&self.peek().ty()) {}
    }
}
