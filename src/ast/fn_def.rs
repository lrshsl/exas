use scope::change_indentation;

use crate::{ast::current_padding, parser::Parser};

use super::*;

#[derive(Debug, Clone)]
pub struct FnSignature<'source> {
    pub params: Vec<Param<'source>>,
}

#[derive(Debug, Clone)]
pub struct FnDef<'source> {
    pub signature: FnSignature<'source>,
    pub body: ListContent<'source>,
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
        writeln!(output, "{}fn [", current_padding())?;
        change_indentation(scope::IndentationChange::More);
        for param in &self.signature.params {
            writeln!(output, "{}{:?},", current_padding(), param)?;
        }
        change_indentation(scope::IndentationChange::Less);
        write!(output, "{}] ", current_padding())?;
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
