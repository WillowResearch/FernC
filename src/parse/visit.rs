use core::fmt;
use std::fmt::{DebugStruct, Write};

use crate::{source_map::Source, utils::tree_writer::TreePrinter};

use super::ast::{
    BlockAst, DeclarationAst, ExpressionAst, ExpressionStatementAst, FileAst, FnArgAst, FnDeclAst,
    FnReturnTypeAst, IfExprAst, LetStatementAst, StatementAst, TypeAnnotationAst, TypeAst,
};

trait AstVisitor<T> {
    fn visit_file(&mut self, file: &FileAst) -> T;
    fn visit_decl(&mut self, decl: &DeclarationAst) -> T;
    fn visit_fn_decl(&mut self, fn_decl: &FnDeclAst) -> T;
    fn visit_fn_arg(&mut self, fn_arg: &FnArgAst) -> T;
    fn visit_fn_ret_ty(&mut self, fn_ret_ty: &Option<FnReturnTypeAst>) -> T;
    fn visit_block(&mut self, block: &BlockAst) -> T;
    fn visit_statement(&mut self, stmt: &StatementAst) -> T;
    fn visit_let_statement(&mut self, let_stmt: &LetStatementAst) -> T;
    fn visit_type_annotation(&mut self, type_annotation: &TypeAnnotationAst) -> T;
    fn visit_expr_stmt(&mut self, expr_stmt: &ExpressionStatementAst) -> T;
    fn visit_expr(&mut self, expr: &ExpressionAst) -> T;
    fn visit_if_expr(&mut self, if_expr: &IfExprAst) -> T;
    fn visit_ty(&mut self, ty: &TypeAst) -> T;
}

pub fn pretty_print(file: &FileAst, source: &Source, wr: &mut dyn fmt::Write) {
    let mut pp = PrettyPrintAst { source };
    let text = pp.visit_file(file);
    write!(wr, "{text}");
}

pub struct PrettyPrintAst<'a> {
    source: &'a Source,
}

impl<'a> AstVisitor<String> for PrettyPrintAst<'a> {
    fn visit_file(&mut self, file: &FileAst) -> String {
        TreePrinter::start("File")
            .field("filename", self.source.filename())
            .field_list("declarations", &file.declarations, |d| self.visit_decl(d))
            .finish()
    }

    fn visit_decl(&mut self, decl: &DeclarationAst) -> String {
        match decl {
            DeclarationAst::FnDecl(fn_decl_ast) => self.visit_fn_decl(fn_decl_ast),
        }
    }

    fn visit_fn_decl(&mut self, fn_decl: &FnDeclAst) -> String {
        TreePrinter::start("FnDecl")
            .field("name", self.source.text_of_span(fn_decl.name_ident))
            .field_list("args", &fn_decl.args, |a| self.visit_fn_arg(a))
            .field("ret_ty", self.visit_fn_ret_ty(&fn_decl.return_ty))
            .field("body", self.visit_block(&fn_decl.body))
            .finish()
    }

    fn visit_fn_arg(&mut self, fn_arg: &FnArgAst) -> String {
        TreePrinter::start("FnArg")
            .field("name", self.source.text_of_span(fn_arg.name))
            .field("ty", self.visit_ty(&fn_arg.ty))
            .finish()
    }

    fn visit_fn_ret_ty(&mut self, fn_ret_ty: &Option<FnReturnTypeAst>) -> String {
        let Some(fn_ret_ty) = fn_ret_ty else {
            return String::from("()")
        };

        self.visit_ty(&fn_ret_ty.ty)
    }

    fn visit_block(&mut self, block: &BlockAst) -> String {
        String::from("Block")
    }

    fn visit_statement(&mut self, stmt: &StatementAst) -> String {
        todo!()
    }

    fn visit_let_statement(&mut self, let_stmt: &LetStatementAst) -> String {
        todo!()
    }

    fn visit_type_annotation(&mut self, type_annotation: &TypeAnnotationAst) -> String {
        todo!()
    }

    fn visit_expr_stmt(&mut self, expr_stmt: &ExpressionStatementAst) -> String {
        todo!()
    }

    fn visit_expr(&mut self, expr: &ExpressionAst) -> String {
        todo!()
    }

    fn visit_if_expr(&mut self, if_expr: &IfExprAst) -> String {
        todo!()
    }

    fn visit_ty(&mut self, ty: &TypeAst) -> String {
        self.source.text_of_span(ty.name_ident).into()
    }
}
