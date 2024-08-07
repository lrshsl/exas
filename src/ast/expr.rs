use super::*;

use super::assign::parse_assign;

#[derive(Clone)]
pub enum Expr {
    FnDef(FnDef),
    FnCall(FnCall),

    Assign(Assign),

    Ident(Ident),
    Int(i32),
    String(&'static str),
}

impl AstNode for Expr {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        match self {
            Expr::FnDef(fn_def) => fn_def.build_context(ctx, current_scope),
            Expr::Assign(assign) => assign.build_context(ctx, current_scope),
            Expr::Ident(ident) => ident.build_context(ctx, current_scope),

            Expr::Int(_) => {}
            Expr::String(_) => {}
            Expr::FnCall(_) => {}
        }
    }

    fn check_and_emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        match self {
            Expr::FnDef(fn_def) => fn_def.check_and_emit(ctx, scope_stack),
            Expr::Ident(ident) => {
                // Check if in scope
                if let Some(name_matches) = ctx.symbols.get(ident.0) {
                    let scope_matches = name_matches.iter().filter(|symbol| {
                        scope_stack.iter().any(|scope_id| scope_id == &symbol.scope)
                    });
                    match scope_matches.count() {
                        0 => panic!(
                            "<{}> not defined in this scope (scope {})",
                            ident.0,
                            scope_stack.last().unwrap()
                        ),
                        1 => ident.check_and_emit(ctx, scope_stack),
                        _ => panic!(
                            "<{}> defined multiple times in this scope (scope {})",
                            ident.0,
                            scope_stack.last().unwrap()
                        ),
                    }
                } else {
                    panic!(
                        "<{}> not defined in this scope (scope {})",
                        ident.0,
                        scope_stack.last().unwrap()
                    );
                }
            }

            Expr::Assign(assign) => assign.check_and_emit(ctx, scope_stack),
            Expr::Int(int) => println!("{}Int({})", current_padding(), int),
            Expr::String(string) => println!("{}String({})", current_padding(), string),
            Expr::FnCall(fn_call) => fn_call.check_and_emit(ctx, scope_stack),
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FnDef(fn_def) => write!(f, "{:?}", fn_def),
            Expr::FnCall(fn_call) => write!(f, "{:?}", fn_call),
            Expr::Assign(assign) => write!(f, "{:?}", assign),
            Expr::Ident(ident) => write!(f, "{:?}", ident),
            Expr::Int(int) => write!(f, "{:?}", int),
            Expr::String(string) => write!(f, "{:?}", string),
        }
    }
}

impl Parsable for Expr {
    /// Should be called when on the first token
    fn parse(parser: &mut Parser) -> Result<Expr, ParsingError> {
        let token = match parser.current_token.as_ref() {
            Some(Ok(token)) => token,
            Some(Err(error)) => return Err(ParsingError::TokenError(*error)),
            None => {
                return Err(ParsingError::AbruptEof(
                    "expr",
                    parser.lexer.extras.clone(),
                    vec![
                        Token::Symbol("("),
                        Token::Ident,
                        Token::Int(0),
                        Token::String,
                        Token::KeywordFn,
                    ],
                ))
            }
        };
        match token {
            Token::Symbol("(") => {
                parser.advance();
                Expr::parse(parser)
            }
            Token::Ident => {
                let ident = parser.current_slice;
                parser.advance();
                match parser.current_token {
                    Some(Ok(Token::Symbol("="))) => {
                        parser.advance();
                        Ok(parse_assign(parser, ident)?)
                    }
                    None | Some(Ok(_)) => {
                        parser.advance();
                        Ok(Expr::FnCall(FnCall {
                            name: ident,
                            args: ArgumentList::parse(parser)?,
                        }))
                    }
                    Some(Err(error)) => Err(ParsingError::TokenError(error)),
                }
            }
            Token::Int(val) => {
                let val = *val;
                parser.advance();
                Ok(Expr::Int(val))
            }
            Token::String => {
                let strval = parser.current_slice;
                parser.advance();
                Ok(Expr::String(strval))
            }
            Token::KeywordFn => {
                parser.advance();
                Ok(Expr::FnDef(FnDef::parse(parser)?))
            }
            _ => Err(ParsingError::UnexpectedToken(
                "expr",
                parser.lexer.extras.clone(),
                token.clone(),
                vec![Token::Ident, Token::Int(0), Token::String, Token::KeywordFn],
            )),
        }
    }
}
