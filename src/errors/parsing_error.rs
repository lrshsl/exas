use crate::lexer::{FileContext, Token};

#[macro_export]
macro_rules! assert_token_matches {
    ($tok:expr, $pattern:pat, $filecontext:expr) => {
        match $tok {
            Some(Ok($pattern)) => {}
            Some(Ok(other_token)) => {
                return Err(ParsingError::UnexpectedToken(
                    "ArgumentList",
                    $filecontext.clone(),
                    other_token,
                    vec![Token::Symbol(')')],
                ))
            }
            Some(Err(())) => {
                return Err(ParsingError::TokenError(format!(
                    "lexer error in {file}@{line}",
                    file = $filecontext.filename,
                    line = $filecontext.line
                )))
            }
            None => {
                return Err(ParsingError::AbruptEof(
                    "ArgumentList",
                    $filecontext.clone(),
                    vec![Token::Symbol(')')],
                ))
            }
        }
    };
}

pub enum ParsingError<'source> {
    AbruptEof(&'source str, FileContext<'source>, Vec<Token>),
    UnexpectedToken(&'source str, FileContext<'source>, Token, Vec<Token>),
    TokenError(String),
}

impl std::fmt::Display for ParsingError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::AbruptEof(msg, context, expected) => {
                write!(
                    f,
                    "AbruptEof(expected one of {expected:?} at {file}@{line}): {msg}",
                    file = context.filename,
                    line = context.line
                )
            }
            ParsingError::UnexpectedToken(msg, context, token, expected) => {
                write!(
                    f,
                    "UnexpectedToken({token:?}, expected one of <{expected:?}> at {file}@{line}): \
                     {msg}",
                    file = context.filename,
                    line = context.line
                )
            }
            ParsingError::TokenError(error) => write!(f, "TokenError({:?})", error),
        }
    }
}
