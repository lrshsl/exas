use super::*;

use super::assign::parse_assign;

#[derive(Clone)]
pub enum Expr<'source> {
    FnDef(FnDef<'source>),
    FnCall(FnCall<'source>),

    Assign(Assign<'source>),

    Int(i32),
    String(&'source str),
}

impl std::fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FnDef(fn_def) => fn_def.fmt(f),
            Expr::FnCall(fn_call) => fn_call.fmt(f),
            Expr::Assign(assign) => assign.fmt(f),
            Expr::Int(int) => write!(f, "Int({})", int),
            Expr::String(string) => write!(f, "String({})", string),
        }
    }
}

impl<'source> AstNode<'source> for Expr<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>) {
        match self {
            Expr::FnDef(fn_def) => fn_def.build_context(ctx, scope_stack),
            Expr::Assign(assign) => assign.build_context(ctx, scope_stack),

            Expr::Int(_) => {}
            Expr::String(_) => {}
            Expr::FnCall(_) => {}
        }
    }

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        match self {
            Expr::FnDef(fn_def) => fn_def.check_and_emit(output, ctx, scope_stack)?,
            Expr::Assign(assign) => assign.check_and_emit(output, ctx, scope_stack)?,
            Expr::Int(int) => write!(output, "{}Int({})", current_padding(), int)?,
            Expr::String(string) => write!(output, "{}String({})", current_padding(), string)?,
            Expr::FnCall(fn_call) => fn_call.check_and_emit(output, ctx, scope_stack)?,
        }
        Ok(())
    }
}

impl<'source> Parsable<'source> for Expr<'source> {
    /// Should be called when on the first token
    fn parse(parser: &mut Parser<'source>) -> Result<Expr<'source>, ParsingError<'source>> {
        let token = match parser.current_token.as_ref() {
            Some(Ok(token)) => token,
            Some(Err(())) => {
                return Err(ParsingError::TokenError(format!(
                    "Lexer error in {file}@{line}",
                    file = parser.lexer.extras.filename,
                    line = parser.lexer.extras.line
                )))
            }
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
                    None | Some(Ok(_)) => Ok(Expr::FnCall(FnCall {
                        name: ident,
                        args: ArgumentList::parse(parser)?,
                    })),
                    Some(Err(())) => Err(ParsingError::TokenError(format!(
                        "Lexer error in {file}@{line}",
                        file = parser.lexer.extras.filename,
                        line = parser.lexer.extras.line,
                    ))),
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
