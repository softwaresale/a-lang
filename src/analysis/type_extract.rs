use std::collections::HashMap;
use crate::frontend::ast::{ArrayAccessNode, AssignmentNode, Ast, BinaryOpNode, CompilationUnitNode, CompositionSpecNode, CondExprNode, FieldDeclarationNode, FunCallNode, FunctionDeclarationNode, IdentNode, LitNode, NamedArgNode, ObjectDeclarationNode, ParamNode, ReturnNode, TypeSpecNode, UnaryOpNode, VariableDeclarationNode, WhileNode};
use crate::frontend::ast::visitor::AstVisitor;
use crate::types::{FunParam, FunType, ObjectType, Type};

/// Pulls type info from an AST node
pub struct TypeExtractor;

impl AstVisitor for TypeExtractor {
    type ResT = Type;
    type ErrT = ();
    type CtxT = ();

    fn visit_compilation_unit(&self, node: CompilationUnitNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(Type::Unknown)
    }

    fn visit_function_declaration(&self, node: FunctionDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let mut params = Vec::<FunParam>::new();
        for param in node.params {
            let param_type = self.visit(param, ctx.clone())?;
            let param_name = node.name.into_ident();
            let param = FunParam {
                tp: param_type.into(),
                name: param_name.ident,
            };
            params.push(param)
        }

        let ret_tp = self.visit(node.ret_tp, ctx)?;

        Ok(Type::Function(FunType {
            ret: ret_tp.into(),
            args: params,
        }))
    }

    fn visit_object_declaration(&self, node: ObjectDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let mut fields = HashMap::<String, Box<Type>>::new();
        for field in node.fields {
            let field_name = field.clone().into_field_decl().name.into_ident().ident;
            let field_type = self.visit(field, ctx.clone())?;
            fields.insert(field_name, field_type.into());
        }

        Ok(Type::Object(ObjectType {
            name: "".to_string(),
            props: fields,
            comps: Default::default(),
        }))
    }

    fn visit_field_declaration(&self, node: FieldDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        self.visit(node.tp, ctx)
    }

    fn visit_composition_spec(&self, node: CompositionSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        // I think this is always going to be a user-defined type...
        Ok(Type::Unknown)
    }

    fn visit_variable_declaration(&self, node: VariableDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        self.visit(node.tp, ctx)
    }

    fn visit_param(&self, node: ParamNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        self.visit(node.tp, ctx)
    }

    fn visit_block(&self, stmts: Vec<Box<Ast>>, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let mut stmts = stmts;
        let Some(last_stmt) = stmts.into_iter().last() else {
            return Ok(Type::Unit)
        };

        self.visit(last_stmt, ctx)
    }

    fn visit_assignment(&self, node: AssignmentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        self.visit(node.rhs, ctx)
    }

    fn visit_unary_op(&self, node: UnaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(Type::Unknown)
    }

    fn visit_binary_op(&self, node: BinaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(Type::Unknown)
    }

    fn visit_cond_expr(&self, node: CondExprNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        // TODO
        // 1. check both branches and find conflicting values
        // 2. only check false in case it's a unit
        self.visit(node.true_branch, ctx)
    }

    fn visit_while(&self, node: WhileNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        // loops return nothing
        // TODO subject to change if I include breaks
        Ok(Type::Unit)
    }

    fn visit_identifier(&self, node: IdentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        // we don't know an identifier's type until we perform symbol lookups
        Ok(Type::Unknown)
    }

    fn visit_literal(&self, node: LitNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(node.lit.into())
    }

    fn visit_fun_call(&self, node: FunCallNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        // function calls are not know syntactically. We have to analyze/infer the return type once
        // we have symbol information. So, we return unknown
        Ok(Type::Unknown)
    }

    fn visit_named_arg(&self, node: NamedArgNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        self.visit(node.value, ctx)
    }

    fn visit_return(&self, node: ReturnNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        self.visit(node.expr, ctx)
    }

    fn visit_array_access(&self, node: ArrayAccessNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let derefed_type = self.visit(node.derefed, ctx)?;
        let inner_type = match derefed_type {
            Type::Array(inner, _) => *inner,
            Type::View(inner) => *inner,
            _ => Type::Unknown
        };

        Ok(inner_type)
    }

    fn visit_type_spec(&self, node: TypeSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(node.tp)
    }
}
