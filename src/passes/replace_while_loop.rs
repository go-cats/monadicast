use crate::monad::ast::Pass;
use crate::MonadicAst;
use syn::{visit::Visit, visit_mut::VisitMut, Expr, ExprBinary, ExprPath, ExprWhile, Stmt};

#[derive(Default)]
pub struct WhileLoopReplacer {}

impl WhileLoopReplacer {
    fn record_if_whileloop(&self, wloop: &ExprWhile) {
        // if we see a while loop, log it
        println!("{:?}", wloop);
        println!("Found a while loop");
    }
}

impl Visit<'_> for WhileLoopReplacer {
    // Inspects while loop and prints if we find one
    fn visit_expr_while(&mut self, whileloop: &ExprWhile) {
        println!("Found a while loop");
        self.record_if_whileloop(whileloop);
    }
}

impl VisitMut for WhileLoopReplacer {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if let Stmt::Expr(Expr::While(while_loop), _) = stmt {
            if let Expr::Binary(cond) = &*while_loop.cond {
                if let (Expr::Path(loop_var), Expr::Path(upper_bound)) = (&*cond.left, &*cond.right)
                {
                    let loop_var_ident = &loop_var.path.segments[0].ident;
                    let upper_bound = upper_bound.clone();
                    let mut loop_body = while_loop.body.clone();

                    // Remove increment/decrement statements from the loop body
                    loop_body.stmts.retain(|stmt| {
                        if let Stmt::Expr(Expr::Assign(assign_op), _) = stmt {
                            if let Expr::Path(ref left_path) = *assign_op.left {
                                if left_path == loop_var {
                                    return false;
                                }
                            }
                        }
                        true
                    });

                    // Create the new for loop statement
                    let for_loop_stmt: Stmt = syn::parse_quote! {
                        for #loop_var_ident in 0..#upper_bound {
                            #loop_body
                        }
                    };

                    *stmt = for_loop_stmt;
                }
            }
        }

        // Visit other statements
        syn::visit_mut::visit_stmt_mut(self, stmt);
    }
}

impl Pass for WhileLoopReplacer {
    fn bind(&mut self, mut monad: MonadicAst) -> MonadicAst {
        // Inspects the while loops in the file and records them
        self.visit_file(&mut monad.ast);

        // TODO - Replaces the while loops with a for loop when appropriate
        self.visit_file_mut(&mut monad.ast);

        monad
    }
}
