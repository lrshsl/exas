use typeexpr::{find_type, TypeExpr};

use super::*;

#[derive(Clone)]
pub struct Param<'source> {
    pub name: Option<&'source str>,
    pub pattern: MatchPattern<'source>,
}

impl<'source> Parsable<'source> for Param<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<Self, ParsingError<'source>> {
        let param_name = match parser.current_token.as_ref() {
            Some(Ok(token)) => match token {
                Token::Ident => Some(parser.current_slice),
                Token::Symbol(":") => None,
                _ => panic!("Don't think that's allowed"),
            },
            _ => todo!("Handle error"),
        };
        parser.advance();
        let param_type = match parser.current_token.as_ref() {
            Some(Ok(token)) => match token {
                Token::Ident => Some(parser.current_slice),
                Token::Symbol("]") => None,
                _ => panic!("Don't think that's allowed"),
            },
            _ => todo!("Handle error"),
        };
        Ok(Param {
            name: param_name,
            pattern: MatchPattern::TypeExpr {
                typename: param_type,
            },
        })
    }
}

impl<'source> CompTimeSize<'source> for Param<'source> {
    fn number_bytes(&self, ctx: &'source ProgramContext) -> usize {
        match &self.pattern {
            MatchPattern::RawToken(raw_token) => raw_token.number_bytes(ctx),
            MatchPattern::TypeExpr { typename } => find_type(ctx, *typename).unwrap().size,
        }
    }
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
    TypeExpr { typename: Option<&'source str> },
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
                Token::Symbol(",") => {
                    return Err(ParsingError::UnexpectedToken(
                        "params",
                        parser.lexer.extras.clone(),
                        token.clone(),
                        vec![],
                    ))
                }
                Token::Symbol("[") => {
                    parser.advance();
                    params.push(Param::parse(parser)?)
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
