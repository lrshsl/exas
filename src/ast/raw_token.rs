use expr::SmallValue;

use crate::lexer::Token;

use super::*;

#[derive(Clone)]
pub enum RawToken<'source> {
    Ident(Ident<'source>),
    Symbol(char),

    Expr(Expr<'source>),
}

impl CompTimeSize<'_> for RawToken<'_> {
    fn number_bytes(&self, ctx: &ProgramContext) -> usize {
        match self {
            Self::Ident(_) => todo!(),
            Self::Symbol(_) => 1,
            Self::Expr(expr) => expr.number_bytes(ctx),
        }
    }
}

impl std::fmt::Debug for RawToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawToken::Ident(ident) => write!(f, "{:?}", ident),
            RawToken::Symbol(symbol) => write!(f, "Symbol({:?})", symbol),
            RawToken::Expr(expr) => write!(f, "{:?}", expr),
        }
    }
}

impl<'source> RawToken<'source> {
    pub fn from_token(token: &Token, slice: &'source str) -> Self {
        match token {
            Token::Ident => RawToken::Ident(Ident(slice)),
            Token::Int(val) => RawToken::Expr(Expr::SmallValue(SmallValue::DWord(*val))),
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
            Some(Ok(Token::String)) => {
                Ok(RawToken::from_token(&Token::String, parser.current_slice))
            }
            Some(Ok(Token::Int(val))) => Ok(RawToken::from_token(
                &Token::Int(*val),
                parser.current_slice,
            )),
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
