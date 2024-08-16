use super::*;

#[derive(Debug, Clone)]
pub struct FnCall<'source> {
    pub name: &'source str,
    pub args: Vec<RawToken<'source>>,
}

impl AstNode<'_> for FnCall<'_> {
    fn build_context(&self, _ctx: &mut ProgramContext, _scope_stack: &mut Vec<ScopeId>) {}

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext<'_>,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        let _function_entry = match ctx.symbols.get(self.name) {
            None => {
                return compile_error(
                    ctx.file_context.clone(),
                    format!("Undefined function: {}", self.name).to_string(),
                )
            }
            Some(global_matches) => match global_matches
                .iter()
                .filter(|f| scope_stack.contains(&f.scope))
                .collect::<Vec<_>>()[..]
            {
                [] => {
                    return compile_error(
                        ctx.file_context.clone(),
                        format!(
                            "Function not defined in this scope: {}, scope: {}",
                            self.name,
                            scope_stack.last().unwrap()
                        ),
                    )
                }
                [one_match] => one_match,
                [..] => {
                    return compile_error(
                        ctx.file_context.clone(),
                        format!(
                            "Function defined multiple times in this scope: {}",
                            self.name
                        ),
                    )
                }
            },
        };
        // Todo: check signature
        writeln!(
            output,
            "{}FnCall({}, {:?})",
            current_padding(),
            self.name,
            self.args
        )?;
        Ok(())
    }
}

pub type ArgumentList<'source> = Vec<RawToken<'source>>;

impl<'source> Parsable<'source> for ArgumentList<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<ArgumentList<'source>, ParsingError<'source>> {
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
