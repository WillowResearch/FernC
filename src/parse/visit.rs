use core::fmt;
use std::fmt::DebugStruct;

use crate::source_map::Source;

use super::ast::{
    BlockAst, DeclarationAst, ExpressionAst, ExpressionStatementAst, FileAst, FnArgAst, FnDeclAst,
    FnReturnTypeAst, IfExprAst, LetStatementAst, StatementAst, TypeAnnotationAst, TypeAst,
};

trait AstVisitor<T> {
    fn visit_file(&mut self, file: &FileAst) -> T;
    fn visit_decl(&mut self, file: &DeclarationAst) -> T;
    fn visit_fn_decl(&mut self, file: &FnDeclAst) -> T;
    fn visit_fn_arg(&mut self, file: &FnArgAst) -> T;
    fn visit_fn_ret_ty(&mut self, file: &FnReturnTypeAst) -> T;
    fn visit_block(&mut self, file: &BlockAst) -> T;
    fn visit_statement(&mut self, file: &StatementAst) -> T;
    fn visit_let_statement(&mut self, file: &LetStatementAst) -> T;
    fn visit_type_annotation(&mut self, file: &TypeAnnotationAst) -> T;
    fn visit_expr_stmt(&mut self, file: &ExpressionStatementAst) -> T;
    fn visit_expr(&mut self, file: &ExpressionAst) -> T;
    fn visit_if_expr(&mut self, file: &IfExprAst) -> T;
    fn visit_ty(&mut self, file: &TypeAst) -> T;
}

pub fn pretty_print(file: &FileAst, source: &Source, fmt: &mut dyn fmt::Write) {
    let mut pp = PrettyPrintAst { source, fmt };
    pp.visit_file(file);
}

pub struct PrettyPrintAst<'a, 'b> {
    source: &'a Source,
    fmt: &'b mut dyn fmt::Write,
}

impl<'a, 'b> AstVisitor<()> for PrettyPrintAst<'a, 'b> {
    fn visit_file(&mut self, file: &FileAst) -> () {
        todo!()
    }

    fn visit_decl(&mut self, file: &DeclarationAst) -> () {
        todo!()
    }

    fn visit_fn_decl(&mut self, file: &FnDeclAst) -> () {
        todo!()
    }

    fn visit_fn_arg(&mut self, file: &FnArgAst) -> () {
        todo!()
    }

    fn visit_fn_ret_ty(&mut self, file: &FnReturnTypeAst) -> () {
        todo!()
    }

    fn visit_block(&mut self, file: &BlockAst) -> () {
        todo!()
    }

    fn visit_statement(&mut self, file: &StatementAst) -> () {
        todo!()
    }

    fn visit_let_statement(&mut self, file: &LetStatementAst) -> () {
        todo!()
    }

    fn visit_type_annotation(&mut self, file: &TypeAnnotationAst) -> () {
        todo!()
    }

    fn visit_expr_stmt(&mut self, file: &ExpressionStatementAst) -> () {
        todo!()
    }

    fn visit_expr(&mut self, file: &ExpressionAst) -> () {
        todo!()
    }

    fn visit_if_expr(&mut self, file: &IfExprAst) -> () {
        todo!()
    }

    fn visit_ty(&mut self, file: &TypeAst) -> () {
        todo!()
    }
}
