use typeexpr::{find_type, TypeExpr};

use crate::errors::compile_error;

use super::*;

#[derive(Clone)]
pub struct Param<'source> {
    pub name: Option<&'source str>,
    pub pattern: MatchPattern<'source>,
}

impl PartialEq for Param<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

/// Called when on the opening bracket '[' of a function definition
/// Thus always returns a Param with pattern == TypeExpr
impl<'source> Parsable<'source> for Param<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<Self, ParsingError<'source>> {
        let param_name = match parser.current_token.as_ref() {
            Some(Ok(token)) => match token {
                Token::Ident => {
                    let param_name = parser.current_slice;
                    parser.advance();
                    Some(param_name)
                }
                Token::Symbol(":") => None,
                _ => panic!("Don't think that's allowed"),
            },
            _ => todo!("Handle error"),
        };
        parser.advance(); // Skip ':'
        let param_type = match parser.current_token.as_ref() {
            Some(Ok(token)) => match token {
                Token::Ident => {
                    let param_type = parser.current_slice;
                    parser.advance();
                    Some(param_type)
                }
                Token::Symbol("]") => None,
                _ => panic!("Don't think that's allowed"),
            },
            _ => todo!("Handle error"),
        };
        parser.advance(); // Skip ']'
        Ok(Param {
            name: param_name,
            pattern: MatchPattern::TypeExpr {
                typename: param_type,
            },
        })
    }
}

impl<'source> CompTimeSize<'source> for Param<'source> {
    fn number_bytes(&self, ctx: &'source ProgramContext) -> ByteSize {
        match &self.pattern {
            MatchPattern::RawToken(raw_token) => raw_token.number_bytes(ctx),
            MatchPattern::TypeExpr { typename } => {
                let Some(type_) = find_type(ctx, *typename) else {
                    panic!("Could not find type: {typename:?}");
                };
                type_.size.clone()
            }
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

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern<'source> {
    RawToken(RawToken<'source>),
    TypeExpr { typename: Option<&'source str> },
}

impl MatchPattern<'_> {
    pub fn matches_arg(&self, ctx: &ProgramContext<'_>, arg: &RawToken<'_>) -> bool {
        match self {
            Self::RawToken(raw_token) => raw_token == arg,
            Self::TypeExpr { typename } => match find_type(ctx, *typename) {
                Some(typeexpr::Type {
                    type_fn: Some(type_fn),
                    size: _,
                }) => type_fn(arg),

                Some(typeexpr::Type {
                    type_fn: None,
                    size: _,
                }) => true,

                None => unreachable!("Type should exist at this point"),
            },
        }
    }
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
