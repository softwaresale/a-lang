pub mod visitor;

use crate::literal::Literal;
use crate::frontend::location::{HasLocation, SourceRange};
use crate::operators::{BinaryOp, UnaryOp};
use crate::types::{Type, VariableDeclarationMode};

#[derive(Debug, Clone)]
pub struct CompilationUnitNode {
    /// list of declarations
    pub(crate) declarations: Vec<Box<Ast>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDeclarationNode {
    /// function name
    pub(crate) name: Box<Ast>,
    /// the parameters given to this function
    pub(crate) params: Vec<Box<Ast>>,
    /// the return type of this function
    pub(crate) ret_tp: Box<Ast>,
    /// The function body
    pub(crate) body: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FieldDeclarationNode {
    /// the name of this field
    pub(crate) name: Box<Ast>,
    /// the given type
    pub(crate) tp: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct CompositionSpecNode {
    /// the type being composed
    pub(crate) composed_type: Box<Ast>,
    /// an optional alias given
    pub(crate) alias: Option<Box<Ast>>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct VariableDeclarationNode {
    /// how this variable is declared
    pub(crate) decl_mode: VariableDeclarationMode,
    /// the name of this variable
    pub(crate) name: Box<Ast>,
    /// the type of this variable
    pub(crate) tp: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct ParamNode {
    /// name of this parameter
    pub(crate) name: Box<Ast>,
    /// the type of this parameter
    pub(crate) tp: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct AssignmentNode {
    pub(crate) decl: Box<Ast>,
    pub(crate) rhs: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct UnaryOpNode {
    pub(crate) op: UnaryOp,
    pub(crate) child: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct BinaryOpNode {
    pub(crate) op: BinaryOp,
    pub(crate) lhs: Box<Ast>,
    pub(crate) rhs: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct FunCallNode {
    /// the function being called
    pub(crate) fun_name: Box<Ast>,
    /// the arguments passed
    pub(crate) args: Vec<Box<Ast>>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct NamedArgNode {
    /// the argument we are passing to
    pub(crate) param_name: Box<Ast>,
    /// the value we are passing
    pub(crate) value: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct WhileNode {
    /// the condition to evaluate each time
    pub(crate) cond: Box<Ast>,
    /// the body of the loop
    pub(crate) body: Box<Ast>,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct IdentNode {
    pub(crate) ident: String,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct LitNode {
    pub(crate) lit: Literal,
    /// location in source where this node occurs
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct ReturnNode {
    pub(crate) expr: Box<Ast>,
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct ArrayAccessNode {
    /// the expression being accessed
    pub(crate) derefed: Box<Ast>,
    /// the expression used to access the array
    pub(crate) access: Box<Ast>,
    /// source location
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
pub struct TypeSpecNode {
    /// the type of this spec
    pub(crate) tp: Type,
    pub(crate) location: SourceRange,
}

#[derive(Debug, Clone)]
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
    Return(ReturnNode),
    ArrayAccess(ArrayAccessNode),
    TypeSpec(TypeSpecNode),
}

impl HasLocation for Ast {
    fn source_range(&self) -> SourceRange {
        match self {
            Ast::CompilationUnit(_) => SourceRange::default(),
            Ast::FunctionDeclaration(node) => node.location,
            Ast::ObjectDeclaration(node) => node.location,
            Ast::FieldDeclaration(node) => node.location,
            Ast::CompositionSpec(node) => node.location,
            Ast::VariableDeclaration(node) => node.location,
            Ast::Param(node) => node.location,
            Ast::Block(node) => {
                let start = node.first().map(|ast| ast.source_range()).unwrap();
                let end = node.last().map(|ast| ast.source_range()).unwrap();
                SourceRange::spanned(&start, &end)
            },
            Ast::Assignment(node) => node.location,
            Ast::UnaryOp(node) => node.location,
            Ast::BinaryOp(node) => node.location,
            Ast::CondExpr(node) => node.location,
            Ast::While(node) => node.location,
            Ast::Identifier(node) => node.location,
            Ast::Literal(node) => node.location,
            Ast::FunCall(node) => node.location,
            Ast::NamedArg(node) => node.location,
            Ast::Return(node) => node.location,
            Ast::ArrayAccess(node) => node.location,
            Ast::TypeSpec(node) => node.location,
        }
    }
}

impl Ast {
    pub fn into_ident(self) -> IdentNode {
        let Self::Identifier(ident_node) = self else {
            panic!("Expected identifier, but something else")
        };
        
        ident_node
    }
    
    pub fn into_field_decl(self) -> FieldDeclarationNode {
        let Self::FieldDeclaration(node) = self else {
            panic!("Expected field decl, but something else")
        };

        node
    }
}
