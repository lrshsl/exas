use scope::change_indentation;

use super::*;

#[derive(Clone)]
pub struct ListContent {
    pub elements: Vec<Expr>,
}

impl std::fmt::Debug for ListContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ListContent {{")?;
        for element in self.elements.iter() {
            writeln!(f, "{:?},", element)?;
        }
        writeln!(f, "}}")
    }
}

impl AstNode for ListContent {
    fn build_context(&self, ctx: &mut ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        scope_stack.push(next_scope());

        for element in self.elements.iter() {
            element.build_context(ctx, scope_stack);
        }
    }

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        // Start a new scope
        writeln!(output, "{}{{", current_padding())?;
        scope_stack.push(next_scope());
        change_indentation(scope::IndentationChange::More);

        for element in &self.elements {
            element.check_and_emit(output, ctx, scope_stack)?;
        }

        change_indentation(scope::IndentationChange::Less);
        scope_stack.pop();
        writeln!(output, "{}}}", current_padding())?;
        Ok(())
    }
}

impl Parsable for ListContent {
    fn parse(parser: &mut Parser) -> Result<ListContent, ParsingError> {
        let mut elements = vec![];
        loop {
            let token = match parser.current_token.as_ref() {
                Some(Ok(token)) => token,
                Some(Err(())) => {
                    return Err(ParsingError::TokenError(format!(
                        "Lexer error in {file}@{line}",
                        file = parser.lexer.extras.file,
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

                Token::Ident | Token::Int(_) | Token::String | Token::KeywordFn => {
                    elements.push(Expr::parse(parser)?)
                }

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
