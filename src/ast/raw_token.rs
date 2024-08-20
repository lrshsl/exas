use crate::lexer::Token;

use super::*;

#[derive(Clone)]
pub enum RawToken<'source> {
    Ident(Ident<'source>),
    Int(i32),
    String(&'source str),
    Symbol(char),

    Expr(Expr<'source>),
}

impl CompTimeSize for RawToken<'_> {
    fn number_bytes(&self) -> usize {
        match self {
            Self::Int(int) => 4,
            Self::Ident(_) => todo!(),
            Self::String(_) => todo!(),
            Self::Symbol(_) => 1,
            Self::Expr(expr) => expr.number_bytes(),
        }
    }
}

impl std::fmt::Debug for RawToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawToken::Ident(ident) => write!(f, "{:?}", ident),
            RawToken::Int(int) => write!(f, "Int({:?})", int),
            RawToken::String(string) => write!(f, "String({:?})", string),
            RawToken::Symbol(symbol) => write!(f, "Symbol({:?})", symbol),
            RawToken::Expr(expr) => write!(f, "{:?}", expr),
        }
    }
}

impl<'source> RawToken<'source> {
    pub fn from_token(token: &Token, slice: &'source str) -> Self {
        match token {
            Token::Ident => RawToken::Ident(Ident(slice)),
            Token::Int(val) => RawToken::Int(*val),
            Token::String => RawToken::String(slice),
            _ => {
                assert_eq!(slice.len(), 1);
                RawToken::Symbol(slice.chars().next().unwrap())
            }
        }
    }
}

impl<'source> Parsable<'source> for RawToken<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<RawToken<'source>, ParsingError<'source>> {
        match parser.current_token.as_ref() {
            Some(Ok(Token::String)) => Ok(RawToken::String(parser.current_slice)),
            Some(Ok(Token::Int(val))) => Ok(RawToken::Int(*val)),
            Some(Ok(Token::Ident)) => Ok(RawToken::Ident(Ident(parser.current_slice))),
            Some(Ok(Token::KeywordFn)) => Ok(RawToken::Ident(Ident("fn"))),
            Some(Ok(Token::Symbol("("))) => Ok(RawToken::Expr(Expr::parse(parser)?)),
            Some(Ok(Token::Symbol(symbol))) => {
                assert_eq!(symbol.len(), 1, "Symbol should be a single character");
                Ok(RawToken::Symbol(symbol.chars().next().unwrap()))
            }
            Some(Err(())) => Err(ParsingError::TokenError(format!(
                "Lexer error in {file}@{line}",
                file = parser.lexer.extras.filename,
                line = parser.lexer.extras.line
            ))),
            None => Err(ParsingError::AbruptEof(
                "raw_token",
                parser.lexer.extras.clone(),
                vec![
                    Token::String,
                    Token::Int(0),
                    Token::Ident,
                    Token::KeywordFn,
                    Token::Symbol("("),
                    Token::Symbol("<AnySymbol>"),
                ],
            )),
            other => panic!("Impossible: {other:?}"),
        }
    }
}
