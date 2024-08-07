use super::*;

#[derive(Debug, Clone)]
pub struct Param {
    pub name: Option<&'static str>,
    pub pattern: MatchPattern,
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    RawToken(RawToken),
    TypeExpr,
}

pub type ParamList = Vec<Param>;

impl Parsable for ParamList {
    fn parse(parser: &mut Parser) -> Result<ParamList, ParsingError> {
        let mut params = Vec::new();
        while let Some(Ok(token)) = parser.current_token.as_ref() {
            match token {
                Token::Symbol("{") => {
                    parser.advance();
                    break;
                }
                Token::Ident => {
                    params.push(Param {
                        name: Some(parser.current_slice),
                        pattern: MatchPattern::RawToken(RawToken::Ident(Ident(
                            parser.current_slice,
                        ))),
                    });
                    parser.advance();
                }
                Token::Int(val) => {
                    params.push(Param {
                        name: None,
                        pattern: MatchPattern::RawToken(RawToken::Int(*val)),
                    });
                    parser.advance();
                }
                // TODO: Add strings etc, and type patterns!
                _ => {
                    return Err(ParsingError::UnexpectedToken(
                        "params",
                        parser.lexer.extras.clone(),
                        token.clone(),
                        vec![Token::Ident, Token::Int(0), Token::Symbol("{")],
                    ))
                }
            }
        }
        Ok(params)
    }
}
