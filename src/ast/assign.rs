use std::{io, ops::Deref, rc::Rc};

use super::*;

#[derive(Debug, Clone)]
pub struct Assign<'source> {
    pub name:  &'source str,
    pub value: Rc<Expr<'source>>,
}

impl PartialEq for Assign<'_> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
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
                writeln!(output, "\n|| Function {name}\n{name}: ", name = self.name)?;
                change_indentation(IndentationChange::More);
                fn_def.check_and_emit(output, ctx, scope_stack)?;
                change_indentation(IndentationChange::Less);
            }
            Expr::FnCall(fn_call) => {
                fn_call.check_and_emit(output, ctx, scope_stack)?;
            }
            Expr::Type(type_) => writeln!(
                output,
                "\n|| Type {type_name}\n{type_name} = type {size}",
                type_name = self.name,
                size = type_.size
            )?,
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
