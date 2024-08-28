use super::SyntaxErrorContext;
use std::io;

pub type CheckResult<T> = Result<T, CheckError>;

pub enum CheckError {
    CompileError(SyntaxErrorContext, String),
    EmitError(io::Error),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CheckError::CompileError(context, msg) => write!(
                f,
                "[Compile Error]<{file} {line}> {msg}\n\t\"{line_content}\"",
                file = context.filename,
                line = context.line,
                line_content = context.line_content
            ),
            CheckError::EmitError(error) => write!(f, "{}", error),
        }
    }
}

impl From<io::Error> for CheckError {
    fn from(error: io::Error) -> Self {
        CheckError::EmitError(error)
    }
}
