use crate::lexer::{FileContext, Token};

pub enum ParsingError<'source> {
    AbruptEof(&'source str, FileContext<'source>, Vec<Token<'source>>),
    UnexpectedToken(
        &'source str,
        FileContext<'source>,
        Token<'source>,
        Vec<Token<'source>>,
    ),
    TokenError(String),
}

impl std::fmt::Debug for ParsingError<'_> {
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
