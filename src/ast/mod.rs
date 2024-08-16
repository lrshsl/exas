use std::io;

use crate::lexer::{FileContext, Token};
use crate::parsing_error::ParsingError;

mod scope;
pub(crate) use scope::ProgramContext;
use scope::{current_padding, next_scope, Symbol};
use scope::{reset_scope_and_indent, ScopeId};

mod expr;
pub use expr::Expr;

mod raw_token;
pub use raw_token::RawToken;

mod ident;
pub use ident::Ident;

mod assign;
pub use assign::Assign;

mod fn_call;
pub use fn_call::{ArgumentList, FnCall};

mod fn_def;
pub use fn_def::FnDef;

mod params;
pub use params::{Param, ParamList};

mod listcontent;
pub use listcontent::ListContent;

use crate::parser::Parser;

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
                "[Compile Error]<{file} {line}> {msg}",
                file = context.filename,
                line = context.line
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

pub fn compile_error<T>(context: FileContext, msg: String) -> CheckResult<T> {
    Err(CheckError::CompileError(
        SyntaxErrorContext {
            line: context.line,
            filename: context.filename,
            line_content: context
                .source
                .lines()
                .nth(context.line - 1)
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

pub trait AstNode<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>);
    fn check_and_emit<Output: io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext<'source>,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()>;
}

pub trait Parsable<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<Self, ParsingError<'source>>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Ast<'source> {
    pub program: ListContent<'source>,
}

impl<'source> Ast<'source> {
    pub fn build_context(&self, ctx: &mut ProgramContext<'source>) {
        reset_scope_and_indent();
        self.program.build_context(ctx, &mut vec![next_scope()]);
    }

    pub fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
    ) -> CheckResult<()> {
        reset_scope_and_indent();
        self.program
            .check_and_emit(output, ctx, &mut vec![next_scope()])
    }
}
