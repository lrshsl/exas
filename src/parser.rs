use crate::{ast::*, lexer::Token};

#[derive(Debug)]
pub enum ParsingError {
    AbruptEof,
    UnexpectedToken(Token<'static>, Vec<Token<'static>>),
    TokenError(<Token<'static> as logos::Logos<'static>>::Error),
}

macro_rules! unpack_token {
    ($token:expr) => {
        match $token {
            Some(Ok(token)) => token,
            Some(Err(error)) => return Err(ParsingError::TokenError(error)),
            None => break,
        }
    };
}

pub struct Parser {
    lexer: logos::Lexer<'static, Token<'static>>,
}

impl Parser {
    pub fn new(lexer: logos::Lexer<'static, Token<'static>>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<Ast, ParsingError> {
        let mut ast = Ast {
            stmts: ListContent { elements: vec![] },
        };
        loop {
            let token = unpack_token!(self.lexer.next());

            match token {
                Token::Symbol(",") => {}
                Token::Int => {
                    ast.stmts.elements.push(Expr::Int(Int(
                        self
                            .lexer
                            .slice()
                            .parse()
                            .expect("Should be able to parse int"),
                    )));
                }
                Token::String => {
                    ast.stmts.elements.push(Expr::String(
                            TokString(self.lexer.slice())));
                }
                Token::Ident => {
                    let ident = self.lexer.slice();
                    match self.lexer.next() {
                        // Ident or a function call without arguments
                        Some(Ok(Token::Symbol(","))) | None => {
                            ast.stmts
                                .elements
                                .push(Expr::Ident(Ident( ident )));
                        }
                        // Function call with arguments
                        Some(Ok(first_arg_token)) => {
                            let args = self.parse_arguments(first_arg_token)?;
                            ast.stmts.elements.push(Expr::FnCall(FnCall {
                                fn_name: ident,
                                args,
                            }));
                        }
                        Some(Err(error)) => return Err(ParsingError::TokenError(error)),
                    }
                }
                _ => {
                    return Err(ParsingError::UnexpectedToken(
                        token,
                        vec![Token::Symbol(","), Token::Int, Token::String, Token::Ident],
                    ))
                }
            }
        }
        Ok(ast)
    }

    pub fn parse_arguments(
        &mut self,
        first_arg_token: Token,
    ) -> Result<Vec<RawToken>, ParsingError> {
        let mut args = Vec::from([RawToken::from_token(first_arg_token, self.lexer.slice())]);
        while let Some(Ok(token)) = self.lexer.next() {
            match token {
                Token::Symbol(",") => break,
                Token::Symbol("(") => args.push(RawToken::Expr(self.parse_expr()?)),
                token => args.push(RawToken::from_token(token, self.lexer.slice())),
            }
        }
        Ok(args)
    }

    fn parse_raw_token(&mut self) -> Result<RawToken, ParsingError> {
        match self.lexer.next() {
            Some(Ok(Token::String)) => Ok(RawToken::String(TokString(self.lexer.slice()))),
            Some(Ok(Token::Int)) => Ok(RawToken::Int(Int(
                self
                    .lexer
                    .slice()
                    .parse()
                    .expect("Should be able to parse int"),
            ))),
            Some(Ok(Token::Ident)) => Ok(RawToken::Ident(Ident(self.lexer.slice()))),
            Some(Ok(Token::Symbol("("))) => Ok(RawToken::Expr(self.parse_expr()?)),
            Some(Ok(Token::Symbol(symbol))) => {
                assert_eq!(symbol.len(), 1, "Symbol should be a single character");
                Ok(RawToken::Symbol(symbol.chars().next().unwrap()))
            }
            Some(Err(error)) => Err(ParsingError::TokenError(error)),
            None => Err(ParsingError::AbruptEof),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParsingError> {
        let token = match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(error)) => return Err(ParsingError::TokenError(error)),
            None => return Err(ParsingError::AbruptEof),
        };
        match token {
            Token::Ident => match self.peek() {
                Some(Ok(Token::Symbol("("))) => Ok(self.parse_fn_call()),
                None | Some(Ok(_)) => Ok(Expr::Ident(Ident(self.lexer.slice()))),
                Some(Err(error)) => Err(ParsingError::TokenError(error)),
            },
            Token::Int => {
                let int = self.lexer.slice().parse().unwrap();
                Ok(Expr::Int(Int(int)))
            }
            _ => Err(ParsingError::UnexpectedToken(
                token,
                vec![Token::Ident, Token::Int],
            )),
        }
    }

    fn parse_fn_call(&mut self) -> Expr {
        let fn_name = self.lexer.slice();
        let args = vec![];

        // TODO: parse args

        Expr::FnCall(FnCall { fn_name, args })
    }

    fn peek(&mut self) -> Option<Result<Token<'static>, <Token<'static> as logos::Logos<'static>>::Error>> {
        self.lexer.clone().next()
    }
}
