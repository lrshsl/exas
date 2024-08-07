use super::*;

#[derive(Clone)]
pub struct FnCall {
    pub name: &'static str,
    pub args: Vec<RawToken>,
}

impl std::fmt::Debug for FnCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnCall({:?}, {:?})", self.name, self.args)
    }
}

impl AstNode for FnCall {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {}

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> std::io::Result<()> {
        writeln!(
            output,
            "{}FnCall({:?}, {:?})",
            current_padding(),
            self.name,
            self.args
        )
    }
}

pub type ArgumentList = Vec<RawToken>;

impl Parsable for ArgumentList {
    fn parse(parser: &mut Parser) -> Result<ArgumentList, ParsingError> {
        let mut args = vec![];
        while let Some(Ok(token)) = parser.current_token.as_ref() {
            match token {
                Token::Symbol(")") | Token::Symbol(",") => {
                    parser.advance();
                    break;
                }
                Token::Symbol("(") => {
                    parser.advance();
                    args.push(RawToken::Expr(Expr::parse(parser)?))
                }
                token => {
                    args.push(RawToken::from_token(token.clone(), parser.current_slice));
                    parser.advance();
                } // TODO: Some other tokens are not allowed here?
            }
        }
        Ok(args)
    }
}
