mod hir;
mod lift_types;
mod type_lifting;
mod type_infer;
mod type_extract;
mod type_check;
mod type_infer;
mod type_check;

use crate::analysis::hir::Hir;
use crate::frontend::ast::Ast;

pub fn analyze_ast(ast: Ast) -> Hir {
    todo!()
}
