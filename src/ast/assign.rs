use fn_call::push_args;
use scope::change_indentation;

use super::*;
use std::rc::Rc;
use std::{io, ops::Deref};

use super::{AstNode, Expr, ProgramContext, ScopeId, Symbol};

#[derive(Debug, Clone)]
pub struct Assign<'source> {
    pub name: &'source str,
    pub value: Rc<Expr<'source>>,
}

impl<'source> AstNode<'source> for Assign<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>) {
        if let Expr::Type(type_) = self.value.as_ref() {
            ctx.types.insert(self.name, type_.clone());
        } else {
            // Other expressions into symbols
            ctx.symbols.entry(self.name).or_default().push(Symbol {
                scope: *scope_stack.last().unwrap(),
                value: Rc::clone(&self.value),
            });
        }
    }

    fn check_and_emit<Output: io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        match self.value.deref() {
            Expr::FnDef(fn_def) => {
                writeln!(output, "{}: ", self.name)?;
                change_indentation(scope::IndentationChange::More);
                fn_def.check_and_emit(output, ctx, scope_stack)?;
                change_indentation(scope::IndentationChange::Less);
            }
            Expr::FnCall(fn_call) => {
                push_args(output, &fn_call.args)?;
                writeln!(output, "call {}", self.name)?;
            }
            Expr::Type(type_) => writeln!(output, "type {}b {}", type_.size, self.name)?,
            Expr::SmallValue(value) => {
                writeln!(
                    output,
                    "move {}b {} -> {}",
                    value.number_bytes(ctx),
                    value,
                    self.name
                )?;
            }
            Expr::Assign(_) => todo!(),
            Expr::Bytes(_) => todo!(),
            Expr::StringSlice(_) => todo!(),
        }
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
