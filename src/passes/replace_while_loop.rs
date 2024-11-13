use crate::monad::ast::Pass;
use crate::MonadicAst;
use std::collections::HashMap;
use syn::{Expr, ExprLit, ExprWhile, Pat, Stmt, visit_mut::VisitMut, visit::Visit};

#[derive(Default)]
pub struct WhileLoopReplacer {
    loop_vars: HashMap<String, ExprLit>, // Properly declare the field within the struct
}

impl WhileLoopReplacer {
    // Records while loop encountered for logging purposes
    fn record_if_whileloop(&mut self, wloop: &ExprWhile) {
        println!("{:?}", wloop);
        println!("Found a while loop");
    }
}

impl Visit<'_> for WhileLoopReplacer {
    // Logs while loops encountered during the visit
    fn visit_expr_while(&mut self, whileloop: &ExprWhile) {
        println!("Found a while loop");
        self.record_if_whileloop(whileloop);
    }
}

impl VisitMut for WhileLoopReplacer {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        // First, check if the statement is a local variable assignment (e.g., `let mut i = 1`)
        if let Stmt::Local(local) = stmt {
            if let Some(local_init) = local.init.as_ref() {
                if let Expr::Lit(lit) = &*local_init.expr {
                    if let Pat::Ident(pat_ident) = &local.pat {
                        let var_name = pat_ident.ident.to_string();
                        // Store the loop variable and its initial value in the HashMap
                        self.loop_vars.insert(var_name, lit.clone());
                    }
                }
            }
        }

        println!("Finished checking variable assignments\n");

        // Then, check if the statement is a `while` loop we want to transform
        if let Stmt::Expr(Expr::While(while_loop), _) = stmt {
            println!("Statement is a while loop: {:?}\n", while_loop);
            if let Expr::Binary(cond) = &*while_loop.cond {
                // Check that the condition matches the pattern `<loop_var> < <upper_bound>`
                if let (Expr::Path(var_path), Expr::Lit(upper_bound)) = (&*cond.left, &*cond.right) {
                    if let Some(var_ident) = var_path.path.get_ident() {
                        let var_name = var_ident.to_string();
                        println!("Checking if initial value exists for string {} in hashmap {:?} that returns {:?}\n", var_name, self.loop_vars, self.loop_vars.get(&var_name));
                        // Check if we have an initial value for this loop variable
                        if let Some(init_value) = self.loop_vars.get(&var_name) {
                            let mut loop_body = while_loop.body.clone();

                            // Remove any increment statements for the loop variable in the body
                            loop_body.stmts.retain(|stmt| {
                                if let Stmt::Expr(Expr::Assign(assign_expr), _) = stmt {
                                    if let Expr::Path(ref left_path) = *assign_expr.left {
                                        return left_path != var_path;
                                    }
                                }
                                true
                            });

                            // Create the new `for` loop statement
                            let for_loop_stmt: Stmt = syn::parse_quote! {
                                for #var_ident in #init_value..#upper_bound {
                                    #loop_body
                                }
                            };

                            // Replace the `while` loop statement with the `for` loop
                            *stmt = for_loop_stmt;

                            // Remove the variable from the HashMap after transformation
                            self.loop_vars.remove(&var_name);
                        }
                    }
                }
            }
        }

        // Continue visiting other statements normally
        syn::visit_mut::visit_stmt_mut(self, stmt);
    }
}

impl Pass for WhileLoopReplacer {
    fn bind(&mut self, mut monad: MonadicAst) -> MonadicAst {
        // Log while loops for inspection
        // self.visit_file(&mut monad.ast);

        // Replace while loops where appropriate
        self.visit_file_mut(&mut monad.ast);

        monad
    }
}
