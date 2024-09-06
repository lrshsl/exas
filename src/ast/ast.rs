use super::*;

#[derive(Debug)]
pub struct Ast<'source> {
    pub program: ListContent<'source>,
}

impl<'source> Ast<'source> {
    pub fn build_context(&self, ctx: &mut ProgramContext<'source>) {
        reset_scope_and_indent();
        self.program.build_context(ctx, &mut vec![next_scope()]);
    }

    pub fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
    ) -> CheckResult<()> {
        reset_scope_and_indent();
        self.program
            .check_and_emit(output, ctx, &mut vec![next_scope()])
    }
}

impl<'source> Parsable<'source> for Ast<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<Ast<'source>, ParsingError<'source>> {
        Ok(Ast {
            program: ListContent::parse(parser)?,
        })
    }
}
