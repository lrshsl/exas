use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FnDef<'source> {
    pub signature: FnSignature<'source>,
    pub body:      ListContent<'source>,
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
        self.body.check_and_emit(output, ctx, scope_stack)?;
        writeln!(output, "{}ret", current_padding())?;
        Ok(())
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
