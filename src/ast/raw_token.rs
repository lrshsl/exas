use crate::lexer::Token;

use super::*;

#[derive(Clone)]
pub enum RawToken {
    Ident(Ident),
    Int(i32),
    String(&'static str),
    Symbol(char),

    Expr(Expr),
}

impl std::fmt::Debug for RawToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawToken::Ident(ident) => write!(f, "{:?}", ident),
            RawToken::Int(int) => write!(f, "{:?}", int),
            RawToken::String(string) => write!(f, "{:?}", string),
            RawToken::Symbol(symbol) => write!(f, "{:?}", symbol),
            RawToken::Expr(expr) => write!(f, "{:?}", expr),
        }
    }
}

impl RawToken {
    pub fn from_token(token: Token, slice: &'static str) -> Self {
        match token {
            Token::Ident => RawToken::Ident(Ident(slice)),
            Token::Int(val) => RawToken::Int(val),
            Token::String => RawToken::String(slice),
            _ => {
                assert_eq!(slice.len(), 1);
                RawToken::Symbol(slice.chars().next().unwrap())
            }
        }
    }
}

impl Parsable for RawToken {
    fn parse(parser: &mut Parser) -> Result<RawToken, ParsingError> {
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
            Some(Err(error)) => Err(ParsingError::TokenError(*error)),
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
