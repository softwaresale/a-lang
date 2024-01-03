mod ast_lower;

use crate::frontend::location::{HasLocation, SourceRange};
use crate::literal::Literal;
use crate::operators::{BinaryOp, UnaryOp};
use crate::types::{FunParam, ObjectType, Type};

pub struct HirNode<InnerT> {
    /// inner node-specific data
    inner: InnerT,
    /// the type given to this HIR node
    ty: Type,
    /// where in source this node is located
    loc: SourceRange,
}

impl<InnerT> HirNode<InnerT> {
    pub fn new_inner(inner: InnerT) -> Self {
        Self {
            inner,
            ty: Type::Unknown,
            loc: SourceRange::default(),
        }
    }

    pub fn with_type(mut self, tp: Type) -> Self {
        self.ty = tp;
        self
    }

    pub fn with_location(mut self, loc: SourceRange) -> Self {
        self.loc = loc;
        self
    }

    pub fn inner(&self) -> &InnerT {
        &self.inner
    }

    pub fn into_inner(self) -> InnerT {
        self.inner
    }
}

pub struct CompilationUnitHIR {
    /// the functions declared in this compilation unit
    functions: Vec<HirNode<FunctionDeclarationHIR>>,
    /// the objects we defined
    objects: Vec<ObjectType>,
}

pub struct FunctionDeclarationHIR {
    /// the name of this given function
    name: String,
    /// the parameters for this function
    params: Vec<FunParam>,
    /// the body for this function
    body: Vec<Hir>,
}

pub struct VariableDeclarationHIR {
    /// name of this variable
    name: String,
    /// the initializer for this variable
    initializer: Box<Hir>,
}

pub struct BlockHIR {
    /// the instructions in a block
    insts: Vec<Hir>
}

pub struct AssignmentHIR {
    /// assignment lhs
    lhs: Box<Hir>,
    /// assignment rhs
    rhs: Box<Hir>,
}

pub struct UnaryOpHIR {
    op: UnaryOp,
    child: Box<Hir>,
}

pub struct BinaryOpHIR {
    op: BinaryOp,
    lhs: Box<Hir>,
    rhs: Box<Hir>,
}

pub struct ConditionHIR {
    /// condition expression
    cond: Box<Hir>,
    true_branch: Vec<Hir>,
    false_branch: Vec<Hir>,
}

pub struct LoopHIR {
    /// the statements to be executed in the loop
    stmts: Vec<Hir>,
}

pub struct FunCallHIR {
    /// the expression being called
    callee: Box<Hir>,
    /// the arguments with the call
    args: Vec<Hir>,
}

pub struct NamedArgHIR {
    /// the name of this argument
    name: String,
    /// the expression passed to this name
    expr: Box<Hir>,
}

pub struct ReturnHIR {
    /// the return value
    value: Box<Hir>,
}

pub struct ArrayAccessHIR {
    /// the expression being accessed
    accessed: Box<Hir>,
    /// the offset passed
    offset: Box<Hir>,
}

/// high-level intermediate representation. HIR is basically an AST transformed into a sequence of
/// instructions that are still fairly high level. They are more designed to capture user intent
pub enum Hir {
    CompilationUnit(HirNode<CompilationUnitHIR>),
    FunctionDeclaration(HirNode<FunctionDeclarationHIR>),
    VariableDeclaration(HirNode<VariableDeclarationHIR>),
    Block(HirNode<BlockHIR>),
    Assignment(HirNode<AssignmentHIR>),
    UnaryOp(HirNode<UnaryOpHIR>),
    BinaryOp(HirNode<BinaryOpHIR>),
    Condition(HirNode<ConditionHIR>),
    Loop(HirNode<LoopHIR>),
    Break(HirNode<()>),  // TODO break with a value...
    Identifier(HirNode<String>),
    Literal(HirNode<Literal>),
    FunCall(HirNode<FunCallHIR>),
    NamedArg(HirNode<NamedArgHIR>),
    Return(HirNode<ReturnHIR>),
    ArrayAccess(HirNode<ArrayAccessHIR>),
}

impl Hir {
    pub fn ty(&self) -> &Type {
        match self {
            Hir::CompilationUnit(node) => &node.ty,
            Hir::FunctionDeclaration(node) => &node.ty,
            Hir::VariableDeclaration(node) => &node.ty,
            Hir::Assignment(node) => &node.ty,
            Hir::UnaryOp(node) => &node.ty,
            Hir::BinaryOp(node) => &node.ty,
            Hir::Condition(node) => &node.ty,
            Hir::Loop(node) => &node.ty,
            Hir::Identifier(node) => &node.ty,
            Hir::Literal(node) => &node.ty,
            Hir::FunCall(node) => &node.ty,
            Hir::Return(node) => &node.ty,
            Hir::ArrayAccess(node) => &node.ty,
            Hir::NamedArg(node) => &node.ty,
            Hir::Block(node) => &node.ty,
            Hir::Break(node) => &node.ty,
        }
    }
}

impl HasLocation for Hir {
    fn source_range(&self) -> SourceRange {
        match self {
            Hir::CompilationUnit(node) => node.loc,
            Hir::FunctionDeclaration(node) => node.loc,
            Hir::VariableDeclaration(node) => node.loc,
            Hir::Assignment(node) => node.loc,
            Hir::UnaryOp(node) => node.loc,
            Hir::BinaryOp(node) => node.loc,
            Hir::Condition(node) => node.loc,
            Hir::Loop(node) => node.loc,
            Hir::Identifier(node) => node.loc,
            Hir::Literal(node) => node.loc,
            Hir::FunCall(node) => node.loc,
            Hir::Return(node) => node.loc,
            Hir::ArrayAccess(node) => node.loc,
            Hir::NamedArg(node) => node.loc,
            Hir::Block(node) => node.loc,
            Hir::Break(node) => node.loc,
        }
    }
}
