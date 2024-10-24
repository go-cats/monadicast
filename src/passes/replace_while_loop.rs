use crate::monad::ast::Pass;
use crate::MonadicAst;
use syn::{FnArg, Ident, Local, Pat, PatIdent, PatType, Type, TypePtr};
use syn::ExprWhile;
#[derive(Default)]
pub struct WhileLoopReplacer{}

impl Pass for WhileLoopReplacer {
    fn bind(&mut self, monad: MonadicAst) -> MonadicAst {
        monad
    }
}

impl WhileLoopReplacer {
    fn record_if_whileloop(&self, pat: Pat, ty: &Type) {
    }
}

