use scope::change_indentation;

use super::*;
use std::io;
use std::rc::Rc;

use super::{AstNode, Expr, ProgramContext, ScopeId, Symbol};

#[derive(Clone, Debug)]
pub struct Assign {
    pub name: &'static str,
    pub value: Rc<Expr>,
}

impl AstNode for Assign {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        let entry = ctx.symbols.entry(self.name).or_insert(Vec::new());

        if entry.iter().any(|symbol| symbol.scope == current_scope) {
            panic!(
                "Assignment of <{}> shadows other variable in scope",
                self.name
            );
        }
        entry.push(Symbol {
            scope: current_scope,
            value: Expr::Assign(self.clone()),
        });
    }

    fn check_and_emit<Output: io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> io::Result<()> {
        writeln!(output, "{}let {} = ", current_padding(), self.name)?;
        change_indentation(scope::IndentationChange::More);
        self.value.check_and_emit(output, ctx, scope_stack)?;
        change_indentation(scope::IndentationChange::Less);
        Ok(())
    }
}

/// Should be called when on the next token after '='
pub fn parse_assign(parser: &mut Parser, name: &'static str) -> Result<Expr, ParsingError> {
    Ok(Expr::Assign(Assign {
        name,
        value: Expr::parse(parser)?.into(),
    }))
}
