use super::*;

type TypeFn = fn(Expr) -> bool;

#[derive(Debug, Clone)]
pub struct Type {
    pub size: usize,
    pub type_fn: Option<TypeFn>,
}

pub fn find_type<'source>(
    ctx: &'source ProgramContext,
    typename: Option<&str>,
) -> Option<&'source Type> {
    ctx.types.get(typename?)
}

impl Parsable<'_> for Type {
    fn parse<'source>(parser: &mut Parser<'source>) -> Result<Type, ParsingError<'source>> {
        match parser.current_token {
            Some(Ok(Token::Int(int))) => Ok(Type {
                size: int as usize,
                type_fn: None,
            }),
            None => Err(ParsingError::AbruptEof(
                "type",
                parser.lexer.extras.clone(),
                vec![Token::Int(0)],
            )),
            Some(Ok(ref token)) => Err(ParsingError::UnexpectedToken(
                "type",
                parser.lexer.extras.clone(),
                token.clone(),
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

#[derive(Debug, Clone)]
pub struct TypeExpr;

impl Parsable<'_> for TypeExpr {
    fn parse<'source>(parser: &mut Parser<'source>) -> Result<TypeExpr, ParsingError<'source>> {
        todo!()
    }
}
