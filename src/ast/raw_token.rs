use super::*;

#[derive(Clone, PartialEq)]
pub enum RawToken<'source> {
    Ident(Ident<'source>),
    Symbol(char),

    Expr(Expr<'source>),
}

impl CompTimeSize<'_> for RawToken<'_> {
    fn number_bytes(&self, ctx: &ProgramContext) -> ByteSize {
        match self {
            Self::Ident(_) => ByteSize::AnySize,
            Self::Symbol(_) => ByteSize::Exact(1),
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
    pub fn from_token(token: Token, slice: &'source str) -> Self {
        match token {
            Token::Ident => RawToken::Ident(Ident(slice)),
            Token::Int(val) => RawToken::Expr(Expr::SmallValue(SmallValue::Untyped(val as u64))),
            Token::Symbol(symbol) => RawToken::Symbol(symbol),
            token => unimplemented!("Is this token allowed? : {token:?}"),
        }
    }
}

impl<'source> Parsable<'source> for RawToken<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<RawToken<'source>, ParsingError<'source>> {
        match parser.current_token {
            Some(Ok(Token::String)) => {
                Ok(RawToken::from_token(Token::String, parser.current_slice))
            }
            Some(Ok(Token::Int(val))) => {
                Ok(RawToken::from_token(Token::Int(val), parser.current_slice))
            }
            Some(Ok(Token::Ident)) => Ok(RawToken::Ident(Ident(parser.current_slice))),
            Some(Ok(Token::KeywordFn)) => Ok(RawToken::Ident(Ident("fn"))),
            Some(Ok(Token::Symbol('('))) => Ok(RawToken::Expr(Expr::parse(parser)?)),
            Some(Ok(Token::Symbol(symbol))) => Ok(RawToken::Symbol(symbol)),
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
                    Token::Symbol('('),
                ],
            )),
            ref other => panic!("Impossible: {other:?}"),
        }
    }
}
