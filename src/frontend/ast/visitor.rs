use crate::frontend::ast::{ArrayAccessNode, AssignmentNode, Ast, BinaryOpNode, CompilationUnitNode, CompositionSpecNode, CondExprNode, FieldDeclarationNode, FunCallNode, FunctionDeclarationNode, IdentNode, LitNode, NamedArgNode, ObjectDeclarationNode, ParamNode, ReturnNode, TypeSpecNode, UnaryOpNode, VariableDeclarationNode, WhileNode};

pub trait AstVisitor {
    type ResT;
    type ErrT;
    type CtxT: Clone;

    fn visit_compilation_unit(&self, node: CompilationUnitNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_function_declaration(&self, node: FunctionDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_object_declaration(&self, node: ObjectDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_field_declaration(&self, node: FieldDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_composition_spec(&self, node: CompositionSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_variable_declaration(&self, node: VariableDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_param(&self, node: ParamNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_block(&self, stmts: Vec<Box<Ast>>, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_assignment(&self, node: AssignmentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_unary_op(&self, node: UnaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_binary_op(&self, node: BinaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_cond_expr(&self, node: CondExprNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_while(&self, node: WhileNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_identifier(&self, node: IdentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_literal(&self, node: LitNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_fun_call(&self, node: FunCallNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_named_arg(&self, node: NamedArgNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_return(&self, node: ReturnNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_array_access(&self, node: ArrayAccessNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;
    fn visit_type_spec(&self, node: TypeSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT>;

    fn visit(&self, ast: Box<Ast>, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        match *ast {
            Ast::CompilationUnit(node) => self.visit_compilation_unit(node, ctx),
            Ast::FunctionDeclaration(node) => self.visit_function_declaration(node, ctx),
            Ast::ObjectDeclaration(node) => self.visit_object_declaration(node, ctx),
            Ast::FieldDeclaration(node) => self.visit_field_declaration(node, ctx),
            Ast::CompositionSpec(node) => self.visit_composition_spec(node, ctx),
            Ast::VariableDeclaration(node) => self.visit_variable_declaration(node, ctx),
            Ast::Param(node) => self.visit_param(node, ctx),
            Ast::Block(node) => self.visit_block(node, ctx),
            Ast::Assignment(node) => self.visit_assignment(node, ctx),
            Ast::UnaryOp(node) => self.visit_unary_op(node, ctx),
            Ast::BinaryOp(node) => self.visit_binary_op(node, ctx),
            Ast::CondExpr(node) => self.visit_cond_expr(node, ctx),
            Ast::While(node) => self.visit_while(node, ctx),
            Ast::Identifier(node) => self.visit_identifier(node, ctx),
            Ast::Literal(node) => self.visit_literal(node, ctx),
            Ast::FunCall(node) => self.visit_fun_call(node, ctx),
            Ast::NamedArg(node) => self.visit_named_arg(node, ctx),
            Ast::Return(node) => self.visit_return(node, ctx),
            Ast::ArrayAccess(node) => self.visit_array_access(node, ctx),
            Ast::TypeSpec(node) => self.visit_type_spec(node, ctx),
        }
    }
}

pub trait AstVisitorMut {
    type ResT;
    type ErrT;

    fn visit_compilation_unit(&mut self, node: CompilationUnitNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_function_declaration(&mut self, node: FunctionDeclarationNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_object_declaration(&mut self, node: ObjectDeclarationNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_field_declaration(&mut self, node: FieldDeclarationNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_composition_spec(&mut self, node: CompositionSpecNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_variable_declaration(&mut self, node: VariableDeclarationNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_param(&mut self, node: ParamNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_block(&mut self, stmts: Vec<Box<Ast>>) -> Result<Self::ResT, Self::ErrT>;
    fn visit_assignment(&mut self, node: AssignmentNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_unary_op(&mut self, node: UnaryOpNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_binary_op(&mut self, node: BinaryOpNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_cond_expr(&mut self, node: CondExprNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_while(&mut self, node: WhileNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_identifier(&mut self, node: IdentNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_literal(&mut self, node: LitNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_fun_call(&mut self, node: FunCallNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_named_arg(&mut self, node: NamedArgNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_return(&mut self, node: ReturnNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_array_access(&mut self, node: ArrayAccessNode) -> Result<Self::ResT, Self::ErrT>;
    fn visit_type_spec(&mut self, node: TypeSpecNode) -> Result<Self::ResT, Self::ErrT>;

    fn visit(&mut self, ast: Box<Ast>) -> Result<Self::ResT, Self::ErrT> {
        match *ast {
            Ast::CompilationUnit(node) => self.visit_compilation_unit(node),
            Ast::FunctionDeclaration(node) => self.visit_function_declaration(node),
            Ast::ObjectDeclaration(node) => self.visit_object_declaration(node),
            Ast::FieldDeclaration(node) => self.visit_field_declaration(node),
            Ast::CompositionSpec(node) => self.visit_composition_spec(node),
            Ast::VariableDeclaration(node) => self.visit_variable_declaration(node),
            Ast::Param(node) => self.visit_param(node),
            Ast::Block(node) => self.visit_block(node),
            Ast::Assignment(node) => self.visit_assignment(node),
            Ast::UnaryOp(node) => self.visit_unary_op(node),
            Ast::BinaryOp(node) => self.visit_binary_op(node),
            Ast::CondExpr(node) => self.visit_cond_expr(node),
            Ast::While(node) => self.visit_while(node),
            Ast::Identifier(node) => self.visit_identifier(node),
            Ast::Literal(node) => self.visit_literal(node),
            Ast::FunCall(node) => self.visit_fun_call(node),
            Ast::NamedArg(node) => self.visit_named_arg(node),
            Ast::Return(node) => self.visit_return(node),
            Ast::ArrayAccess(node) => self.visit_array_access(node),
            Ast::TypeSpec(node) => self.visit_type_spec(node),
        }
    }
}
