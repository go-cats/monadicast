//! Addresses https://github.com/go-cats/monadicast/issues/8
//!
//! Removes useless identifier expressions, e.g. x;

use crate::monad::ast::Pass;
use crate::MonadicAst;
use syn::visit_mut::VisitMut;
use syn::{Block, Expr, ExprPath, PathArguments, Stmt};

#[derive(Default)]
pub struct IdentifierExpressionRemover;

impl VisitMut for IdentifierExpressionRemover {
    fn visit_block_mut(&mut self, block: &mut Block) {
        let mut to_remove = Vec::new();
        for (i, statement) in block.stmts.iter_mut().enumerate() {
            if let Stmt::Expr(Expr::Path(ExprPath { attrs, qself, path }), semicolon) = statement {
                if attrs.is_empty()
                    && qself.is_none()
                    && path.leading_colon.is_none()
                    && path.segments.len() == 1
                {
                    if path.segments[0].arguments == PathArguments::None {
                        to_remove.push(i);
                    }
                }
            }
        }
        block.stmts = block
            .stmts
            .iter()
            .enumerate()
            .filter(|(i, _)| !to_remove.contains(i))
            .map(|(_, stmt)| stmt.clone())
            .collect();
        syn::visit_mut::visit_block_mut(self, block);
    }
}

impl Pass for IdentifierExpressionRemover {
    fn bind(&mut self, mut monad: MonadicAst) -> MonadicAst {
        self.visit_file_mut(&mut monad.ast);
        monad
    }
}
