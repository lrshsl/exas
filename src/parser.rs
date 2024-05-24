use crate::{
    ast::{Ast, Expr, Statement},
    lexer::Token,
};

#[derive(Debug)]
pub enum ParsingError {
    AbruptEof,
    UnexpectedToken(Token),
    TokenError(<Token as logos::Logos<'static>>::Error),
}

pub struct Parser {
    lexer: logos::Lexer<'static, Token>,
}

impl Parser {
    pub fn new(lexer: logos::Lexer<'static, Token>) -> Self {
        Self { lexer }
    }

    // First function to be called after the constructor
    pub fn parse(&mut self) -> Result<Ast, ParsingError> {
        let mut ast = Ast { stmts: vec![] };
        loop {
            let token = self.lexer.next();
            match token {
                Some(Ok(Token::Newline)) => {
                    continue;
                }
                Some(Ok(Token::Set)) => {
                    // Skip the 'set'
                    match self.lexer.next() {
                        Some(Ok(Token::Ident)) => {}
                        Some(Err(error)) => return Err(ParsingError::TokenError(error)),
                        Some(Ok(token)) => return Err(ParsingError::UnexpectedToken(token)),
                        None => return Err(ParsingError::AbruptEof),
                    }

                    // Parse the ident
                    let ident = self.lexer.slice().to_string();

                    // Skip the '='
                    match self.lexer.next() {
                        Some(Ok(Token::Eq)) => {}
                        Some(Err(error)) => return Err(ParsingError::TokenError(error)),
                        Some(Ok(token)) => return Err(ParsingError::UnexpectedToken(token)),
                        None => return Err(ParsingError::AbruptEof),
                    }

                    let expr = self.parse_expr().unwrap();
                    ast.stmts.push(Statement::Set(ident, expr));
                }
                Some(Ok(Token::Print)) => {
                    let expr = self.parse_expr().unwrap();
                    ast.stmts.push(Statement::Print(expr));
                },
                None => break,
                _ => {}
            }

            // Skip the newline
            match self.lexer.next() {
                Some(Ok(Token::Newline)) => {}
                Some(Err(error)) => return Err(ParsingError::TokenError(error)),
                Some(Ok(token)) => return Err(ParsingError::UnexpectedToken(token)),
                None => return Err(ParsingError::AbruptEof),
            }
        }
        Ok(ast)
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParsingError> {
        match self.lexer.next() {
            Some(Ok(token)) => match token {
                Token::Ident => Ok(Expr::Ident(self.lexer.slice().to_string())),
                Token::Int => Ok(Expr::Int(self.lexer.slice().parse::<i32>().unwrap())),
                _ => return Err(ParsingError::UnexpectedToken(token)),
            },
            Some(Err(error)) => return Err(ParsingError::TokenError(error)),
            None => return Err(ParsingError::AbruptEof),
        }
    }
}
