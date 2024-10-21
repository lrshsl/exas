use super::*;
use crate::assert_token_matches;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Type {
    pub size:    ByteSize,
    pub type_fn: Option<TypeFn>,
}

impl Parsable<'_> for Type {
    fn parse<'source>(parser: &mut Parser<'source>) -> Result<Self, ParsingError<'source>> {
        let size = if let Some(Ok(Token::Int(first))) = parser.current_token {
            parser.advance();
            if let Some(Ok(Token::Int(second))) = parser.current_token {
                ByteSize::Range((first as usize)..(second as usize))
            } else {
                ByteSize::Exact(first.try_into().unwrap())
            }
        } else {
            assert_token_matches!(
                parser.current_token,
                Token::Int(_) | Token::Ident,
                parser.lexer.extras
            );
            ByteSize::AnySize
        };
        let type_fn = None;
        Ok(Self { size, type_fn })
    }
}
