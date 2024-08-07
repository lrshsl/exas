use crate::{ast::current_padding, parser::Parser};

use super::*;

#[derive(Debug, Clone)]
pub struct FnSignature {
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub signature: FnSignature,
    pub body: ListContent,
}

impl AstNode for FnDef {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: usize) {
        self.body.build_context(ctx, next_scope());
    }

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> std::io::Result<()> {
        writeln!(
            output,
            "{}fn {:?} => ",
            current_padding(),
            self.signature.params
        )?;
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
