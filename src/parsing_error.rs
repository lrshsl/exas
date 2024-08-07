use crate::{
    lexer::{FileContext, Token},
    parser::LogosError,
};

pub enum ParsingError {
    AbruptEof(&'static str, FileContext, Vec<Token<'static>>),
    UnexpectedToken(
        &'static str,
        FileContext,
        Token<'static>,
        Vec<Token<'static>>,
    ),
    TokenError(LogosError),
}

impl std::fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::AbruptEof(msg, context, expected) => {
                write!(
                    f,
                    "AbruptEof(expected one of {:?} at {file}@{line}): {msg}",
                    expected,
                    file = context.filename,
                    line = context.line
                )
            }
            ParsingError::UnexpectedToken(msg, context, token, expected) => {
                write!(
                    f,
                    "UnexpectedToken({:?}, expected one of <{:?}> at {file}@{line}): {msg}",
                    token,
                    expected,
                    file = context.filename,
                    line = context.line
                )
            }
            ParsingError::TokenError(error) => write!(f, "TokenError({:?})", error),
        }
    }
}
