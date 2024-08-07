use std::io;

pub(self) use crate::lexer::Token;
pub(self) use crate::parsing_error::ParsingError;

mod scope;
pub(crate) use scope::ProgramContext;
use scope::ScopeId;
pub(self) use scope::{current_padding, next_scope, Symbol};

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

pub trait AstNode {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId);
    fn check_and_emit<Output: io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> io::Result<()>;
}

pub trait Parsable {
    fn parse(parser: &mut Parser) -> Result<Self, ParsingError>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Ast {
    pub stmts: ListContent,
}

impl Ast {
    pub fn print(&self) {
        for stmt in &self.stmts.elements {
            println!("{:?}", stmt);
        }
    }
}

impl AstNode for Ast {
    fn build_context(&self, ctx: &mut ProgramContext, _: ScopeId) {
        self.stmts.build_context(ctx, 0);
    }

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> std::io::Result<()> {
        self.stmts.check_and_emit(output, ctx, scope_stack)
    }
}
