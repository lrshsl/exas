use std::io;

use crate::lexer::FileContext;

use super::{CheckError, CheckResult, ParsingError};

pub type CompileResult<'source, T> = Result<T, CompileError<'source>>;

pub enum CompileError<'source> {
    ParsingError(ParsingError<'source>),
    IoError(io::Error),
}

impl std::fmt::Display for CompileError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompileError::ParsingError(error) => write!(f, "{}", error),
            CompileError::IoError(error) => write!(f, "{}", error),
        }
    }
}

impl<'source> From<ParsingError<'source>> for CompileError<'source> {
    fn from(e: ParsingError<'source>) -> Self {
        CompileError::ParsingError(e)
    }
}

impl From<io::Error> for CompileError<'_> {
    fn from(e: io::Error) -> Self {
        CompileError::IoError(e)
    }
}

pub fn compile_error<T>(context: FileContext, msg: String) -> CheckResult<T> {
    Err(CheckError::CompileError(
        SyntaxErrorContext {
            line: context.line,
            filename: context.filename,
            line_content: context
                .source
                .lines()
                .nth(context.line)
                .expect("Not a valid source line")
                .to_string(),
        },
        msg,
    ))
}

pub struct SyntaxErrorContext {
    pub filename: String,
    pub line: usize,
    pub line_content: String,
}
