use crate::monad::ast::Pass;
use crate::MonadicAst;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::ExprWhile;
use syn::Stmt;
use syn::Expr;

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
        if let Stmt::Expr(Expr::While(ExprWhile { cond, body, .. }), _) = stmt {
            println!("Found a while loop!\n");

            if let Expr::Binary(syn::ExprBinary { left, op: syn::BinOp::Lt(_), right, .. }) = &**cond {
                println!("Found a while loop with a less than operator!\n");

                if let (Expr::Path(loop_variable), Expr::Path(upper_bound)) = (&**left, &**right) {
                    println!("Found a while loop with a less than operator and a loop variable and upper bound!\n");

                    let loop_variable_ident = &loop_variable.path.segments[0].ident;
                    let upper_bound = upper_bound.clone();

                    // Clone the body and remove increment statements
                    let mut loop_body = body.clone();
                    loop_body.stmts.retain(|stmt| {
                        // Check if the statement is an increment of the loop variable using `+=`
                        if let Stmt::Expr(Expr::AssignOp(assign_op), _) = stmt {
                            if let Expr::Path(ref left_path) = *assign_op.left {
                                if left_path == loop_variable && matches!(assign_op.op, syn::BinOp::AddEq(_)) {
                                    // This is an increment (like `i += 1`), so we skip it
                                    return false;
                                }
                            }
                        }
                        true // Keep all other statements
                    });

                    // Create the `for` loop with the modified body
                    let for_loop_stmt: Stmt = syn::parse_quote! {
                        for #loop_variable_ident in 0..#upper_bound {
                            #loop_body
                        }
                    };

                    *stmt = for_loop_stmt;
                    return;
                }
            }
        }

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
