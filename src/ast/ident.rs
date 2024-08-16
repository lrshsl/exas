use crate::ast::current_padding;

use super::{AstNode, CheckResult, ProgramContext, ScopeId};

#[derive(Debug, Clone)]
pub struct Ident<'source>(pub &'source str);

impl std::fmt::Display for Ident<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ident<{}>", self.0)
    }
}

impl AstNode<'_> for Ident<'_> {
    fn build_context(&self, _ctx: &mut ProgramContext, _scope_stack: &mut Vec<ScopeId>) {}

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        if let Some(name_matches) = ctx.symbols.get(self.0) {
            let mut scope_matches = name_matches
                .iter()
                .filter(|symbol| scope_stack.iter().any(|scope_id| scope_id == &symbol.scope));
            match scope_matches.next() {
                Some(first_match) => write!(
                    output,
                    "{}Ident({}) found: {:?}",
                    current_padding(),
                    self.0,
                    first_match
                )?,
                None => panic!("<{}> not defined in this scope", self.0),
            }
            if let Some(second_match) = scope_matches.next() {
                panic!(
                    "<{}> defined multiple times in this scope (scope {})",
                    self.0, second_match.scope
                );
            }
        }
        Ok(())
    }
}
