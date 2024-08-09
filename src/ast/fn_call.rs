use super::*;

#[derive(Clone)]
pub struct FnCall {
    pub name: &'static str,
    pub args: Vec<RawToken>,
}

impl std::fmt::Debug for FnCall {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "FnCall ({name}({args:?})",
            name = self.name,
            args = self.args
        )
    }
}

impl AstNode for FnCall {
    fn build_context(&self, _ctx: &mut ProgramContext, _current_scope: ScopeId) {}

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> std::io::Result<()> {
        let _function_entry = match ctx.symbols.get(self.name) {
            None => panic!("Undefined function: {}", self.name),
            Some(global_matches) => match global_matches
                .iter()
                .filter(|f| scope_stack.contains(&f.scope))
                .collect::<Vec<_>>()[..]
            {
                [] => panic!(
                    "Function not defined in this scope: {}, scope: {}",
                    self.name,
                    scope_stack.last().unwrap()
                ),
                [one_match] => one_match,
                [..] => panic!(
                    "Function defined multiple times in this scope: {}",
                    self.name
                ),
            },
        };
        // Todo: check signature
        writeln!(
            output,
            "{}FnCall({}, {:?})",
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
                    args.push(RawToken::from_token(token, parser.current_slice));
                    parser.advance();
                } // TODO: Some other tokens are not allowed here?
            }
        }
        Ok(args)
    }
}
