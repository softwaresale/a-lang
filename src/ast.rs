use crate::literal::Literal;
use crate::location::SourceRange;
use crate::operators::{BinaryOp, UnaryOp};
use crate::types::{Type, VariableDeclarationMode};

#[derive(Debug)]
pub struct CompilationUnitNode {
    /// list of declarations
    pub(crate) declarations: Vec<Box<Ast>>,
}

#[derive(Debug)]
pub struct FunctionDeclarationNode {
    /// function name
    pub(crate) name: Box<Ast>,
    /// the parameters given to this function
    pub(crate) params: Vec<Box<Ast>>,
    /// the return type of this function
    pub(crate) ret_tp: Type,
    /// The function body
    pub(crate) body: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct ObjectDeclarationNode {
    /// the name of this type
    pub(crate) name: Box<Ast>,
    /// list of composition specifications
    pub(crate) composition_specs: Vec<Box<Ast>>,
    /// the fields found in this type
    pub(crate) fields: Vec<Box<Ast>>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct FieldDeclarationNode {
    /// the name of this field
    pub(crate) name: Box<Ast>,
    /// the given type
    pub(crate) tp: Type,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct CompositionSpecNode {
    /// the type being composed
    pub(crate) composed_type: Box<Ast>,
    /// an optional alias given
    pub(crate) alias: Option<Box<Ast>>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct VariableDeclarationNode {
    /// how this variable is declared
    pub(crate) decl_mode: VariableDeclarationMode,
    /// the name of this variable
    pub(crate) name: Box<Ast>,
    /// the type of this variable
    pub(crate) tp: Type,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct ParamNode {
    /// name of this parameter
    pub(crate) name: Box<Ast>,
    /// the type of this parameter
    pub(crate) tp: Type,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct AssignmentNode {
    pub(crate) decl: Box<Ast>,
    pub(crate) rhs: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct UnaryOpNode {
    pub(crate) op: UnaryOp,
    pub(crate) child: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct BinaryOpNode {
    pub(crate) op: BinaryOp,
    pub(crate) lhs: Box<Ast>,
    pub(crate) rhs: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct CondExprNode {
    /// the condition to evaluation
    pub(crate) cond: Box<Ast>,
    /// the true branch
    pub(crate) true_branch: Box<Ast>,
    /// the false branch
    pub(crate) false_branch: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct FunCallNode {
    /// the function being called
    pub(crate) fun_name: Box<Ast>,
    /// the arguments passed
    pub(crate) args: Vec<Box<Ast>>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct NamedArgNode {
    /// the argument we are passing to
    pub(crate) param_name: Box<Ast>,
    /// the value we are passing
    pub(crate) value: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct WhileNode {
    /// the condition to evaluate each time
    pub(crate) cond: Box<Ast>,
    /// the body of the loop
    pub(crate) body: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct IdentNode {
    pub(crate) ident: String,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct LitNode {
    pub(crate) lit: Literal,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub struct ReturnNode {
    pub(crate) expr: Box<Ast>,
    pub(crate) location: SourceRange,
}

#[derive(Debug)]
pub enum Ast {
    CompilationUnit(CompilationUnitNode),
    FunctionDeclaration(FunctionDeclarationNode),
    ObjectDeclaration(ObjectDeclarationNode),
    FieldDeclaration(FieldDeclarationNode),
    CompositionSpec(CompositionSpecNode),
    VariableDeclaration(VariableDeclarationNode),
    Param(ParamNode),
    Block(Vec<Box<Ast>>),
    Assignment(AssignmentNode),
    UnaryOp(UnaryOpNode),
    BinaryOp(BinaryOpNode),
    CondExpr(CondExprNode),
    While(WhileNode),
    Identifier(IdentNode),
    Literal(LitNode),
    FunCall(FunCallNode),
    NamedArg(NamedArgNode),
    Return(ReturnNode)
}
