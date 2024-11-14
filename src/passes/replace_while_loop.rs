use crate::monad::ast::Pass;
use crate::MonadicAst;
use prettyplease::unparse;
use quote::ToTokens;
use std::collections::HashMap;
use syn::{visit::Visit, visit_mut::VisitMut, Expr, ExprLit, ExprWhile, Lit, LitInt, Pat, Stmt};

#[derive(Default)]
pub struct WhileLoopReplacer {
    loop_vars: HashMap<String, i32>, // Properly declare the field within the struct
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
        if let Stmt::Local(local) = stmt {
            let mut variable_name = String::new();
            if let Pat::Type(t) = &local.pat {
                if let Pat::Ident(ident) = &*t.pat {
                    variable_name = ident.ident.to_string();
                }
            }
            if let Some(local_init) = local.init.as_ref() {
                if let Expr::Cast(cast_expr) = &*local_init.expr {
                    if let Expr::Lit(lit) = &*cast_expr.expr {
                        if let ExprLit {
                            lit: Lit::Int(int_lit),
                            ..
                        } = lit
                        {
                            let int_lit = int_lit.base10_parse::<i32>().unwrap();

                            self.loop_vars.insert(variable_name.clone(), int_lit);
                        }
                    }
                }
            }
        }

        // println!("loop_vars: {:?}\n", self.loop_vars);

        // Then, check if the statement is a `while` loop we want to transform
        if let Stmt::Expr(Expr::While(while_loop), _) = stmt {
            // println!("WHILE LOOP COND: {}\n\n", while_loop.cond.to_token_stream());
            if let Expr::Binary(cond) = &*while_loop.cond {
                // Check that the condition matches the pattern `<loop_var> < <upper_bound>`
                println!("LEFT: {}\n\n", cond.left.to_token_stream());
                println!("RIGHT: {}\n\n", cond.right.to_token_stream());
                println!("BODY: {}\n\n", while_loop.body.to_token_stream());

                // if the condition is a binary expression, we want to replace it with a for loop

                if let Expr::Path(left) = &*cond.left {
                    // println!("path: {}", left.to_token_stream());
                    if let Expr::Path(right) = &*cond.right {
                        // println!("lit: {}", right.to_token_stream());

                        let l_var = left.path.segments[0].ident.to_string();
                        let r_var = right.path.segments[0].ident.to_string();

                        println!("l_var: {:?}", l_var);
                        println!("r_var: {:?}", r_var);

                        let contained = self.loop_vars.contains_key(&l_var);

                        println!("contained: {:?}", contained);

                        // if we have a loop variable, we can replace the while loop with a for loop using the boundary stored in the hashmap
                        if contained {
                            let upper_bound = self.loop_vars.get(&l_var).unwrap();
                            let lower_bound = 0;

                            let for_loop = syn::parse_quote! {
                                for #l_var in #lower_bound..#r_var {
                                }
                            };

                            // Replace the while loop with the for loop
                            *stmt = syn::Stmt::Expr(for_loop, None);
                        }
                    }
                }
            }

            // Continue visiting other statements normally
            syn::visit_mut::visit_stmt_mut(self, stmt);
        }
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
