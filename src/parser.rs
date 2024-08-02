use crate::{ast::*, lexer::Token};

pub enum ParsingError {
    AbruptEof,
    UnexpectedToken(Token<'static>, Vec<Token<'static>>),
    TokenError(<Token<'static> as logos::Logos<'static>>::Error),
}

impl std::fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::AbruptEof => write!(f, "AbruptEof"),
            ParsingError::UnexpectedToken(token, expected) => {
                write!(f, "UnexpectedToken({:?}, {:?})", token, expected)
            }
            ParsingError::TokenError(error) => write!(f, "TokenError({:?})", error),
        }
    }
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
        Ok(Ast {
            stmts: self.parse_listcontent()?,
        })
    }

    fn parse_listcontent(&mut self) -> Result<ListContent, ParsingError> {
        let mut stmts = vec![];
        loop {
            let token = unpack_token!(self.lexer.next());

            match token {
                Token::Symbol(",") => {}
                Token::Int => stmts.push(Expr::Int(Int(self.lexer.slice().parse().unwrap()))),
                Token::String => stmts.push(Expr::String(self.lexer.slice())),
                Token::KeywordFn => stmts.push(Expr::FnDef(self.parse_fn_def()?)),
                Token::Ident => {
                    let ident = self.lexer.slice();
                    match self.lexer.next() {
                        // Ident or a function call without arguments
                        Some(Ok(Token::Symbol(","))) | None => stmts.push(Expr::FnCall(FnCall {
                            fn_name: ident,
                            args: vec![],
                        })),
                        // Assignment
                        Some(Ok(Token::Symbol("="))) => stmts.push(Expr::Assign(Assign {
                            name: ident,
                            value: self.parse_expr()?.into(),
                        })),
                        // Function call with arguments
                        Some(Ok(first_arg_token)) => {
                            let args = self.parse_arguments(first_arg_token)?;
                            stmts.push(Expr::FnCall(FnCall {
                                fn_name: ident,
                                args,
                            }));
                        }
                        Some(Err(error)) => return Err(ParsingError::TokenError(error)),
                    }
                }
                Token::Symbol("}") | Token::Symbol("]") => break,
                _ => {
                    return Err(ParsingError::UnexpectedToken(
                        token,
                        vec![
                            Token::Symbol(","),
                            Token::Int,
                            Token::String,
                            Token::KeywordFn,
                            Token::Ident,
                            Token::Symbol("}"),
                            Token::Symbol("]"),
                        ],
                    ))
                }
            }
        }
        Ok(ListContent { elements: stmts })
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

    fn parse_fn_def(&mut self) -> Result<FnDef, ParsingError> {
        let params = self.parse_params()?;
        let body = self.parse_listcontent()?;
        Ok(FnDef {
            signature: FnSignature { params },
            body,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, ParsingError> {
        let mut params = Vec::new();
        while let Some(Ok(token)) = self.lexer.next() {
            match token {
                Token::Symbol("{") => break,
                Token::Ident => params.push(Param {
                    name: self.lexer.slice(),
                    pattern: MatchPattern::RawToken(RawToken::Ident(Ident(self.lexer.slice()))),
                }),
                Token::Int => params.push(Param {
                    name: self.lexer.slice(),
                    pattern: MatchPattern::RawToken(RawToken::Int(Int(self
                        .lexer
                        .slice()
                        .parse()
                        .expect("Should be able to parse int")))),
                }),
                // TODO: Add strings etc, and type patterns!
                _ => {
                    return Err(ParsingError::UnexpectedToken(
                        token,
                        vec![Token::Ident, Token::Int, Token::Symbol("{")],
                    ))
                }
            }
        }
        Ok(params)
    }

    fn parse_pattern(&mut self) -> Result<MatchPattern, ParsingError> {
        match self.lexer.next() {
            Some(Ok(Token::Ident)) => Ok(MatchPattern::RawToken(RawToken::Ident(Ident(
                self.lexer.slice(),
            )))),
            Some(Ok(Token::Int)) => Ok(MatchPattern::RawToken(RawToken::Int(Int(self
                .lexer
                .slice()
                .parse()
                .expect("Should be able to parse int"))))),
            Some(Ok(Token::String)) => {
                Ok(MatchPattern::RawToken(RawToken::String(self.lexer.slice())))
            }
            Some(Ok(Token::Symbol("("))) => {
                Ok(MatchPattern::RawToken(RawToken::Expr(self.parse_expr()?)))
            }
            Some(Ok(Token::Symbol(symbol))) => {
                assert_eq!(symbol.len(), 1, "Symbol should be a single character");
                Ok(MatchPattern::RawToken(RawToken::Symbol(
                    symbol.chars().next().unwrap(),
                )))
            }
            Some(Ok(token)) => Err(ParsingError::UnexpectedToken(
                token,
                vec![
                    Token::Ident,
                    Token::Int,
                    Token::String,
                    Token::Symbol("("),
                    Token::Symbol(")"),
                ],
            )),
            Some(Err(error)) => Err(ParsingError::TokenError(error)),
            None => Err(ParsingError::AbruptEof),
        }
    }

    fn parse_raw_token(&mut self) -> Result<RawToken, ParsingError> {
        match self.lexer.next() {
            Some(Ok(Token::String)) => Ok(RawToken::String(self.lexer.slice())),
            Some(Ok(Token::Int)) => Ok(RawToken::Int(Int(self
                .lexer
                .slice()
                .parse()
                .expect("Should be able to parse int")))),
            Some(Ok(Token::Ident)) => Ok(RawToken::Ident(Ident(self.lexer.slice()))),
            Some(Ok(Token::KeywordFn)) => Ok(RawToken::Ident(Ident("fn"))),
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
            // TODO: Refine
            Token::Ident => match self.peek() {
                Some(Ok(Token::Symbol("("))) => Ok(self.parse_fn_call()),
                None | Some(Ok(_)) => Ok(Expr::Ident(Ident(self.lexer.slice()))),
                Some(Err(error)) => Err(ParsingError::TokenError(error)),
            },
            Token::Int => {
                let int = self
                    .lexer
                    .slice()
                    .parse()
                    .expect("Should be able to parse int");
                Ok(Expr::Int(Int(int)))
            }
            Token::String => Ok(Expr::String(self.lexer.slice())),
            Token::KeywordFn => Ok(Expr::FnDef(self.parse_fn_def()?)),
            _ => Err(ParsingError::UnexpectedToken(
                token,
                vec![Token::Ident, Token::Int, Token::String, Token::KeywordFn],
            )),
        }
    }

    fn parse_fn_call(&mut self) -> Expr {
        let fn_name = self.lexer.slice();
        let args = vec![];

        // TODO: parse args

        Expr::FnCall(FnCall { fn_name, args })
    }

    fn peek(
        &mut self,
    ) -> Option<Result<Token<'static>, <Token<'static> as logos::Logos<'static>>::Error>> {
        self.lexer.clone().next()
    }
}
