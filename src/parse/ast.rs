use crate::source_map::Span;

#[derive(Debug)]
pub struct FileAst {
    pub declarations: Vec<DeclarationAst>,
}

#[derive(Debug)]
pub enum DeclarationAst {
    FnDecl(FnDeclAst),
}

#[derive(Debug)]
pub struct FnDeclAst {
    pub fn_kw: Span,
    pub name_ident: Span,
    pub args: Vec<FnArgAst>,
    pub return_ty: Option<FnReturnTypeAst>,
    pub body: BlockAst,
}

#[derive(Debug)]
pub struct FnArgAst {
    pub name: Span,
    pub colon: Span,
    pub ty: TypeAst,
}


#[derive(Debug)]
pub struct FnReturnTypeAst {
    pub r_arrow: Span,
    pub ty: TypeAst,
}

#[derive(Debug)]
pub struct BlockAst {
    pub statements: Vec<StatementAst>,
    pub return_expr: Option<ExpressionAst>,
}

#[derive(Debug)]
pub enum StatementAst {
    Semicolon(Span),
    LetStatement(LetStatementAst),
    ExpressionStatement(ExpressionStatementAst),
}

#[derive(Debug)]
pub struct LetStatementAst {
    pub let_kw: Span,
    pub name_ident: Span,
    pub type_annotation: Option<TypeAnnotationAst>,
    pub equals: Span,
    pub value: ExpressionAst,
    pub semicolon: Span,
}

#[derive(Debug)]
pub struct TypeAnnotationAst {
    pub colon: Span,
    pub ty: TypeAst,
}

#[derive(Debug)]
pub struct ExpressionStatementAst {
    pub expr: ExpressionAst,
    pub semicolon: Option<Span>,
}

#[derive(Debug)]
pub enum ExpressionAst {}

#[derive(Debug)]
pub struct IfExprAst {
    pub if_kw: Span,
    pub condition: ExpressionAst,
    pub body: BlockAst,
}

#[derive(Debug)]
pub struct TypeAst {
    pub name_ident: Span
}
