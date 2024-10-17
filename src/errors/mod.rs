mod check_result;
mod compile_result;
mod parsing_error;

pub(crate) use check_result::{CheckError, CheckResult};
pub(crate) use compile_result::{compile_error, CompileResult, SyntaxErrorContext};
pub(crate) use parsing_error::ParsingError;
