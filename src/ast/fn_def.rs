use crate::{ast::current_padding, parser::Parser};

use super::*;

pub struct Register(u8);

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.0)
    }
}

pub fn free_register() -> Register {
    return Register(0);
}

#[derive(Debug, Clone)]
pub struct FnSignature<'source> {
    pub params: Vec<Param<'source>>,
}

#[derive(Debug, Clone)]
pub struct FnDef<'source> {
    pub signature: FnSignature<'source>,
    pub body: ListContent<'source>,
}

fn stack_pop_remaining_parameters<Output: std::io::Write>(
    ctx: &ProgramContext,
    output: &mut Output,
    params: &Vec<Param>,
) -> CheckResult<()> {
    for param in params {
        match param.number_bytes(ctx) {
            //todo
            _ => writeln!(
                output,
                "{}pop<{}b> -> {}       | argument",
                current_padding(),
                param.number_bytes(ctx),
                free_register()
            )?,
        }
    }
    Ok(())
}

impl<'source> AstNode<'source> for FnDef<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>) {
        self.body.build_context(ctx, scope_stack);
    }

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        // TODO: pass first parameters through registers
        stack_pop_remaining_parameters(ctx, output, &self.signature.params)?;
        writeln!(output, "{}ret\n", current_padding())?;
        self.body.check_and_emit(output, ctx, scope_stack)
    }
}

impl<'source> Parsable<'source> for FnDef<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<FnDef<'source>, ParsingError<'source>> {
        let params = ParamList::parse(parser)?;
        let body = ListContent::parse(parser)?;
        Ok(FnDef {
            signature: FnSignature { params },
            body,
        })
    }
}
