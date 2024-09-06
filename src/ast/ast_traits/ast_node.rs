use std::io;

use super::*;

pub trait AstNode<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>);
    fn check_and_emit<Output: io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext<'source>,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()>;
}
