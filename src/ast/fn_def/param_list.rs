use super::*;

pub type ParamList<'source> = Vec<Param<'source>>;

impl<'source> Parsable<'source> for ParamList<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<ParamList<'source>, ParsingError<'source>> {
        let mut params = Vec::new();
        while let Some(Ok(token)) = parser.current_token.as_ref() {
            match token {
                Token::Symbol("{") => {
                    parser.advance();
                    break;
                }
                Token::Symbol(",") => {
                    return Err(ParsingError::UnexpectedToken(
                        "params",
                        parser.lexer.extras.clone(),
                        token.clone(),
                        vec![Token::Symbol("{")],
                    ))
                }
                Token::Symbol("[") => {
                    parser.advance(); // Skip '['
                    params.push(Param::ParamExpr(ParamExpr::parse(parser)?))
                }
                token => {
                    params.push(Param::LiteralMatcher(RawToken::from_token(
                        token,
                        parser.lexer.slice(),
                    )));
                    parser.advance();
                }
            }
        }
        Ok(params)
    }
}
