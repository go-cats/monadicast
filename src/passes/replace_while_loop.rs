use crate::monad::ast::Pass;
use crate::MonadicAst;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::ExprWhile;

#[derive(Default)]
pub struct WhileLoopReplacer {}

impl WhileLoopReplacer {
    fn record_if_whileloop(&self, wloop: &ExprWhile) {
        // if we see a while loop, log it
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
    // TODO
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
