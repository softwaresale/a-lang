use crate::analysis::type_extract::TypeExtractor;
use crate::frontend::ast::{ArrayAccessNode, AssignmentNode, Ast, BinaryOpNode, CompilationUnitNode, CompositionSpecNode, CondExprNode, FieldDeclarationNode, FunCallNode, FunctionDeclarationNode, IdentNode, LitNode, NamedArgNode, ObjectDeclarationNode, ParamNode, ReturnNode, TypeSpecNode, UnaryOpNode, VariableDeclarationNode, WhileNode};
use crate::frontend::ast::visitor::{AstVisitor};
use crate::symtab::{Symbol, SymbolTable};
use crate::types::Type;

pub struct TypeLifter {
    extractor: TypeExtractor,
}

impl TypeLifter {
    pub fn new() -> Self {
        Self {
            extractor: TypeExtractor
        }
    }
}

impl AstVisitor for TypeLifter {
    type ResT = SymbolTable;
    type ErrT = ();
    type CtxT = SymbolTable;

    fn visit_compilation_unit(&self, node: CompilationUnitNode, mut ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        for decl in node.declarations {
            ctx = self.visit(decl, ctx)?;
        }

        Ok(ctx)
    }

    fn visit_function_declaration(&self, node: FunctionDeclarationNode, mut ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {

        let loc = node.location.clone();
        let name = node.name.into_ident().ident;

        let function_type = self.extractor.visit_function_declaration(node, ())?;

        let symbol = Symbol {
            name,
            tp: function_type,
            loc,
        };

        ctx.add_symbol(symbol);

        Ok(ctx)
    }

    fn visit_object_declaration(&self, node: ObjectDeclarationNode, mut ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let loc = node.location.clone();
        let Type::Object(function_type) = self.extractor.visit_object_declaration(node, ())? else {
            panic!()
        };

        let name = function_type.name.clone();

        let symbol = Symbol {
            name,
            tp: Type::Object(function_type),
            loc,
        };

        ctx.add_symbol(symbol);

        Ok(ctx)
    }

    fn visit_field_declaration(&self, _node: FieldDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_composition_spec(&self, _node: CompositionSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_variable_declaration(&self, _node: VariableDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_param(&self, _node: ParamNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_block(&self, _stmts: Vec<Box<Ast>>, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_assignment(&self, _node: AssignmentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_unary_op(&self, _node: UnaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_binary_op(&self, _node: BinaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_cond_expr(&self, _node: CondExprNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_while(&self, _node: WhileNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_identifier(&self, _node: IdentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_literal(&self, _node: LitNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_fun_call(&self, _node: FunCallNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_named_arg(&self, _node: NamedArgNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_return(&self, _node: ReturnNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_array_access(&self, _node: ArrayAccessNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }

    fn visit_type_spec(&self, _node: TypeSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(ctx)
    }
}
