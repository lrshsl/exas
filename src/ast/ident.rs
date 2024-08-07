use crate::ast::current_padding;

use super::{AstNode, ProgramContext, ScopeId};

#[derive(Debug, Clone)]
pub struct Ident(pub &'static str);

impl AstNode for Ident {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {}

    fn check_and_emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        if let Some(name_matches) = ctx.symbols.get(self.0) {
            let mut scope_matches = name_matches
                .iter()
                .filter(|symbol| scope_stack.iter().any(|scope_id| scope_id == &symbol.scope));
            match scope_matches.next() {
                Some(first_match) => println!(
                    "{}Ident({}) found: {:?}",
                    current_padding(),
                    self.0,
                    first_match
                ),
                None => panic!("<{}> not defined in this scope", self.0),
            }
            if let Some(second_match) = scope_matches.next() {
                panic!(
                    "<{}> defined multiple times in this scope (scope {})",
                    self.0, second_match.scope
                );
            }
        }
    }
}
