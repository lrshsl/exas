use scope::change_indentation;

use crate::{ast::current_padding, parser::Parser};

use super::*;

#[derive(Clone)]
pub struct FnSignature {
    pub params: Vec<Param>,
}

#[derive(Clone)]
pub struct FnDef {
    pub signature: FnSignature,
    pub body: ListContent,
}

impl std::fmt::Debug for FnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FnDef (Params: {params:?} {body:?})",
            params = self.signature.params,
            body = self.body
        )
    }
}

impl AstNode for FnDef {
    fn build_context(&self, ctx: &mut ProgramContext, scope_stack: &mut Vec<ScopeId>) {
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
        write!(output, "{}]", current_padding())?;
        self.body.check_and_emit(output, ctx, scope_stack)
    }
}

impl Parsable for FnDef {
    fn parse(parser: &mut Parser) -> Result<FnDef, ParsingError> {
        let params = ParamList::parse(parser)?;
        let body = ListContent::parse(parser)?;
        Ok(FnDef {
            signature: FnSignature { params },
            body,
        })
    }
}
