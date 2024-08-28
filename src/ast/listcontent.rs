use scope::change_indentation;

use super::*;

#[derive(Clone, PartialEq)]
pub struct ListContent<'source> {
    pub elements: Vec<Expr<'source>>,
}

impl std::fmt::Debug for ListContent<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.elements.fmt(f)
    }
}

impl<'source> AstNode<'source> for ListContent<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>) {
        scope_stack.push(next_scope());

        for element in self.elements.iter() {
            element.build_context(ctx, scope_stack);
        }
    }

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext<'source>,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        // Start a new scope
        scope_stack.push(next_scope());

        for element in &self.elements {
            element.check_and_emit(output, ctx, scope_stack)?;
        }

        scope_stack.pop();
        Ok(())
    }
}

impl<'source> Parsable<'source> for ListContent<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<ListContent<'source>, ParsingError<'source>> {
        let mut elements = vec![];
        loop {
            let token = match parser.current_token.as_ref() {
                Some(Ok(token)) => token,
                Some(Err(())) => {
                    return Err(ParsingError::TokenError(format!(
                        "Lexer error in {file}@{line}",
                        file = parser.lexer.extras.filename,
                        line = parser.lexer.extras.line,
                    )))
                }
                None => break,
            };
            match token {
                Token::Symbol(",") => parser.advance(),
                Token::Symbol("]") | Token::Symbol("}") => {
                    parser.advance();
                    break;
                }

                Token::Ident
                | Token::Int(_)
                | Token::String
                | Token::KeywordFn
                | Token::KeywordType => elements.push(Expr::parse(parser)?),

                Token::Symbol(_) => {
                    return Err(ParsingError::UnexpectedToken(
                        "listcontent",
                        parser.lexer.extras.clone(),
                        token.clone(),
                        vec![
                            Token::Symbol(","),
                            Token::Symbol("]"),
                            Token::Symbol("}"),
                            Token::Ident,
                            Token::Int(0),
                            Token::String,
                            Token::KeywordFn,
                        ],
                    ))
                }
                other => panic!("Impossible: {other:?}"),
            }
        }
        Ok(ListContent { elements })
    }
}
