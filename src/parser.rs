use logos::Logos;

use crate::{ast::Ast, errors::ParsingError, lexer::Token};

pub type LogosError<'source> = <Token<'source> as Logos<'source>>::Error;
pub type LogosLexer<'source> = logos::Lexer<'source, Token<'source>>;

pub trait Parsable<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<Self, ParsingError<'source>>
    where
        Self: Sized;
}

pub struct Parser<'source> {
    pub lexer: LogosLexer<'source>,
    pub current_token: Option<Result<Token<'source>, LogosError<'source>>>,
    pub current_slice: &'source str,
}

impl<'source> Parser<'source> {
    pub fn new(lexer: LogosLexer<'source>) -> Self {
        Self {
            lexer,
            current_token: None,
            current_slice: "",
        }
    }

    pub fn parse(&mut self) -> Result<Ast<'source>, ParsingError<'source>> {
        self.advance();
        Ast::parse(self)
    }

    pub fn advance(&mut self) {
        self.current_token = self.lexer.next();
        self.current_slice = self.lexer.slice();
    }
}
