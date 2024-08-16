use scope::change_indentation;

use super::*;
use std::io;
use std::rc::Rc;

use super::{AstNode, Expr, ProgramContext, ScopeId, Symbol};

#[derive(Debug, Clone)]
pub struct Assign<'source> {
    pub name: &'source str,
    pub value: Rc<Expr<'source>>,
}

impl<'source> AstNode<'source> for Assign<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>) {
        ctx.symbols.entry(self.name).or_default().push(Symbol {
            scope: *scope_stack.last().unwrap(),
            value: Rc::clone(&self.value),
        })
    }

    fn check_and_emit<Output: io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        writeln!(output, "{}let {} = ", current_padding(), self.name)?;
        change_indentation(scope::IndentationChange::More);
        self.value.check_and_emit(output, ctx, scope_stack)?;
        change_indentation(scope::IndentationChange::Less);
        Ok(())
    }
}

/// Should be called when on the next token after '='
pub fn parse_assign<'source>(
    parser: &mut Parser<'source>,
    name: &'source str,
) -> Result<Expr<'source>, ParsingError<'source>> {
    Ok(Expr::Assign(Assign {
        name,
        value: Expr::parse(parser)?.into(),
    }))
}
