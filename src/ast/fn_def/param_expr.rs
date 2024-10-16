use super::*;
use crate::assert_token_matches;

#[derive(Debug, Clone)]
pub struct ParamExpr<'source> {
    pub name:     Option<&'source str>,
    pub typename: Option<&'source str>,
}

impl PartialEq for ParamExpr<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Ignore param name
        self.typename == other.typename
    }
}

impl<'source> CompTimeSize<'source> for ParamExpr<'source> {
    fn number_bytes(&self, ctx: &ProgramContext) -> ByteSize {
        let Some(typename) = self.typename else {
            return ByteSize::AnySize;
        };
        if let Some(type_) = find_type(ctx, typename) {
            type_.size.clone()
        } else {
            unreachable!("Type should exist at this point")
        }
    }
}

impl ParamExpr<'_> {
    pub fn matches_arg(&self, ctx: &ProgramContext<'_>, arg: &Expr<'_>) -> bool {
        match self.typename {
            Some(typename) => match find_type(ctx, typename) {
                Some(typeexpr::Type { type_fn, size }) => {
                    size.overlap(&arg.number_bytes(ctx)).is_some()
                        && match type_fn {
                            Some(func) => func(arg),
                            None => true,
                        }
                }
                None => unreachable!("Type should exist at this point"),
            },
            None => true, // I guess? TODO: Type inference
        }
    }
}

impl<'source> Parsable<'source> for ParamExpr<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<Self, ParsingError<'source>> {
        let name = match parser.current_token {
            Some(Ok(token)) => match token {
                Token::Symbol(':') | Token::Symbol(']') => None,
                Token::Ident => {
                    let param_name = Some(parser.current_slice);
                    parser.advance();
                    param_name
                }
                _ => todo!("Handle error"),
            },
            _ => todo!("Handle error"),
        };

        let typename = match parser.current_token {
            Some(Ok(Token::Symbol(']'))) => None,
            Some(Ok(Token::Symbol(':'))) => {
                parser.advance(); // Skip ':'
                match parser.current_token {
                    Some(Ok(Token::Ident)) => {
                        let typename = Some(parser.current_slice);
                        parser.advance();
                        typename
                    }
                    other => panic!("Don't think {other:?} is allowed here (typeexpr)"),
                }
            }
            _ => todo!(),
        };

        assert_token_matches!(
            parser.current_token,
            Token::Symbol(']'),
            parser.lexer.extras
        );
        parser.advance(); // Skip ']'
        Ok(ParamExpr { name, typename })
    }
}
