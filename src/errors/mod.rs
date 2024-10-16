mod check_result;
mod compile_result;
mod parsing_error;

pub use check_result::{CheckError, CheckResult};
pub use compile_result::{compile_error, CompileResult, SyntaxErrorContext};
pub use parsing_error::ParsingError;
