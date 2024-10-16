use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
    pub size:    ByteSize,
    pub type_fn: Option<TypeFn>,
}

impl Parsable<'_> for Type {
    fn parse<'source>(parser: &mut Parser<'source>) -> Result<Type, ParsingError<'source>> {
        match parser.current_token {
            // TODO: Add support for type ranges and type check fns
            Some(Ok(Token::Int(int))) => {
                parser.advance();
                Ok(Type {
                    size:    ByteSize::Exact(int as usize),
                    type_fn: None,
                })
            }
            None => Err(ParsingError::AbruptEof(
                "type",
                parser.lexer.extras.clone(),
                vec![Token::Int(0)],
            )),
            Some(Ok(token)) => Err(ParsingError::UnexpectedToken(
                "type",
                parser.lexer.extras.clone(),
                token,
                vec![Token::Int(0)],
            )),
            Some(Err(())) => Err(ParsingError::TokenError(format!(
                "Parsing error in {file}@{line}",
                file = parser.lexer.extras.filename,
                line = parser.lexer.extras.line
            ))),
        }
    }
}
