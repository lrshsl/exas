use std::fmt::Formatter;

use scope::change_indentation;

use super::*;

#[derive(Debug, Clone)]
pub struct ListContent {
    pub elements: Vec<Expr>,
}

impl AstNode for ListContent {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        let new_scope = next_scope();

        for element in self.elements.iter() {
            element.build_context(ctx, new_scope);
        }
    }

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> std::io::Result<()> {
        // Start a new scope
        writeln!(output, "{}{{", current_padding())?;
        scope_stack.push(next_scope());
        change_indentation(scope::IndentationChange::More);

        for element in &self.elements {
            element.check_and_emit(output, ctx, scope_stack)?;
        }

        change_indentation(scope::IndentationChange::Less);
        scope_stack.pop();
        writeln!(output, "{}}}", current_padding())
    }
}

impl Parsable for ListContent {
    fn parse(parser: &mut Parser) -> Result<ListContent, ParsingError> {
        let mut elements = vec![];
        loop {
            let token = match parser.current_token.as_ref() {
                Some(Ok(token)) => token,
                Some(Err(error)) => return Err(ParsingError::TokenError(*error)),
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
