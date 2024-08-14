use logos::Logos;

use crate::{ast::*, lexer::Token, parsing_error::ParsingError};

pub type LogosError = <Token<'static> as Logos<'static>>::Error;
pub type LogosLexer = logos::Lexer<'static, Token<'static>>;

pub struct Parser {
    pub lexer: LogosLexer,
    pub current_token: Option<Result<Token<'static>, LogosError>>,
    pub current_slice: &'static str,
}

impl Parser {
    pub fn new(lexer: LogosLexer) -> Self {
        Self {
            lexer,
            current_token: None,
            current_slice: "",
        }
    }

    pub fn parse(&mut self) -> Result<Ast, ParsingError> {
        self.advance();
        Ok(Ast {
            program: ListContent::parse(self)?,
        })
    }

    pub fn advance(&mut self) {
        self.current_token = self.lexer.next();
        self.current_slice = self.lexer.slice();
    }
}
