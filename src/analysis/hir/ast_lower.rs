use std::marker::PhantomData;
use crate::analysis::hir::{ConditionHIR, FunCallHIR, Hir, HirNode, LoopHIR, NamedArgHIR, ReturnHIR};
use crate::error::source::SourceError;
use crate::frontend::ast::{ArrayAccessNode, AssignmentNode, Ast, BinaryOpNode, CompilationUnitNode, CompositionSpecNode, CondExprNode, FieldDeclarationNode, FunCallNode, FunctionDeclarationNode, IdentNode, LitNode, NamedArgNode, ObjectDeclarationNode, ParamNode, ReturnNode, TypeSpecNode, UnaryOpNode, VariableDeclarationNode, WhileNode};
use crate::frontend::ast::visitor::AstVisitor;
use crate::literal::Literal;
use crate::symtab::SymbolTable;
use crate::types::Type;

pub struct AstLowering<'symtab> {
    phantom: PhantomData<&'symtab SymbolTable>,
}

impl<'symtab> AstVisitor for AstLowering<'symtab> {
    type ResT = Hir;
    type ErrT = SourceError;
    type CtxT = ();

    fn visit_compilation_unit(&self, node: CompilationUnitNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_function_declaration(&self, node: FunctionDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_object_declaration(&self, node: ObjectDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_field_declaration(&self, node: FieldDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_composition_spec(&self, node: CompositionSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_variable_declaration(&self, node: VariableDeclarationNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_param(&self, node: ParamNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_block(&self, stmts: Vec<Box<Ast>>, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_assignment(&self, node: AssignmentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_unary_op(&self, node: UnaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_binary_op(&self, node: BinaryOpNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_cond_expr(&self, node: CondExprNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_while(&self, node: WhileNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        // first, build out a conditional
        let lowered_cond = self.visit(node.cond, ctx)?;
        let check_inst = Hir::Condition(HirNode {
            inner: ConditionHIR {
                cond: lowered_cond.into(),
                true_branch: vec![
                    Hir::Break(HirNode {
                        inner: (),
                        ty: Type::Unknown,
                        loc: Default::default(),
                    })
                ],
                false_branch: vec![
                    Hir::Literal(HirNode {
                        inner: Literal::Unit,
                        ty: Type::Unknown,
                        loc: Default::default(),
                    })
                ],
            },
            ty: Type::Unknown,
            loc: Default::default(),
        });

        let Hir::Block(block_insts) = self.visit(node.body, ctx)? else {
            panic!()
        };

        let mut insts = vec![check_inst];
        insts.extend(block_insts.into_inner().insts.into_iter());

        Ok(Hir::Loop(HirNode {
            inner: LoopHIR {
                stmts: insts,
            },
            ty: Type::Unknown,
            loc: node.location,
        }))
    }

    fn visit_identifier(&self, node: IdentNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        Ok(Hir::Identifier(HirNode {
            inner: node.ident,
            ty: Type::Unknown,
            loc: node.location
        }))
    }

    fn visit_literal(&self, node: LitNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let ty = Type::from(node.lit.clone());
        Ok(Hir::Literal(HirNode {
            inner: node.lit,
            ty,
            loc: node.location,
        }))
    }

    fn visit_fun_call(&self, node: FunCallNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let callee = self.visit(node.fun_name, ctx)?;

        let mut lowered_args = Vec::<Hir>::new();
        for arg in node.args {
            let lowered = self.visit(arg, ctx)?;
            lowered_args.push(lowered);
        }

        Ok(Hir::FunCall(HirNode {
            inner: FunCallHIR {
                callee: callee.into(),
                args: lowered_args,
            },
            ty: Type::Unknown,
            loc: node.location,
        }))
    }

    fn visit_named_arg(&self, node: NamedArgNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let name = node.param_name.into_ident().ident;
        let value = self.visit(node.value, ctx)?;
        Ok(Hir::NamedArg(HirNode {
            inner: NamedArgHIR {
                name,
                expr: value.into(),
            },
            ty: Type::Unknown,
            loc: node.location,
        }))
    }

    fn visit_return(&self, node: ReturnNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        let lowered_expr = self.visit(node.expr, ctx)?;
        let ty = lowered_expr.ty().clone();
        Ok(Hir::Return(HirNode {
            inner: ReturnHIR {
                value: lowered_expr.into(),
            },
            ty,
            loc: node.location,
        }))
    }

    fn visit_array_access(&self, node: ArrayAccessNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        todo!()
    }

    fn visit_type_spec(&self, node: TypeSpecNode, ctx: Self::CtxT) -> Result<Self::ResT, Self::ErrT> {
        unimplemented!()
    }
}
