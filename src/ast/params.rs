use super::*;

#[derive(Clone)]
pub struct Param<'source> {
    pub name: Option<&'source str>,
    pub pattern: MatchPattern<'source>,
}

impl std::fmt::Debug for Param<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name {
            write!(f, "Param<{:?}: {:?}>", name, self.pattern)
        } else {
            write!(f, "Param<{:?}>", self.pattern)
        }
    }
}

#[derive(Debug, Clone)]
pub enum MatchPattern<'source> {
    RawToken(RawToken<'source>),
    TypeExpr,
}

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
                Token::Symbol("[") => todo!(),
                Token::Symbol(",") => {
                    return Err(ParsingError::UnexpectedToken(
                        "params",
                        parser.lexer.extras.clone(),
                        token.clone(),
                        vec![],
                    ))
                }
                token => {
                    params.push(Param {
                        name: None,
                        pattern: MatchPattern::RawToken(RawToken::from_token(
                            token,
                            parser.lexer.slice(),
                        )),
                    });
                    parser.advance();
                }
            }
        }
        Ok(params)
    }
}
