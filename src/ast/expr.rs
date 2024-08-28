use std::{fmt, ops};

use super::*;

use super::assign::parse_assign;

#[derive(Clone, Debug, PartialEq)]
pub enum SmallValue {
    Byte(u8),
    Word(u16),
    DWord(u32),
    QWord(u64),
    Untyped(u64),
}

impl fmt::Display for SmallValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SmallValue::Byte(val) => write!(f, "{:#02X}", val),
            SmallValue::Word(val) => write!(f, "{:#04X}", val),
            SmallValue::DWord(val) => write!(f, "{:#08X}", val),
            SmallValue::QWord(val) | SmallValue::Untyped(val) => write!(f, "{:#016X}", val),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ByteSize {
    Exact(usize),
    Range(ops::Range<usize>),
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteSize::Exact(val) => write!(f, "{}b", val),
            ByteSize::Range(ops::Range { start, end }) => {
                write!(f, "({start}..{end})b")
            }
        }
    }
}

impl CompTimeSize<'_> for SmallValue {
    fn number_bytes(&self, _: &ProgramContext) -> ByteSize {
        match self {
            Self::Byte(_) => ByteSize::Exact(1),
            Self::Word(_) => ByteSize::Exact(2),
            Self::DWord(_) => ByteSize::Exact(4),
            Self::QWord(_) => ByteSize::Exact(8),
            Self::Untyped(_) => ByteSize::Range(1..8),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Expr<'source> {
    FnDef(FnDef<'source>),
    FnCall(FnCall<'source>),
    Type(typeexpr::Type),

    Assign(Assign<'source>),

    SmallValue(SmallValue),

    Bytes(Box<[u8]>),
    StringSlice(&'source str),
}

impl std::fmt::Debug for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FnDef(fn_def) => fn_def.fmt(f),
            Expr::FnCall(fn_call) => fn_call.fmt(f),
            Expr::Type(type_) => type_.fmt(f),
            Expr::Assign(assign) => assign.fmt(f),
            Expr::SmallValue(value) => write!(f, "{:?}", value),
            Expr::Bytes(bytes) => {
                write!(f, "0x")?;
                for byte in bytes.iter() {
                    write!(f, "{:02X}", byte)?;
                }
                Ok(())
            }
            Expr::StringSlice(string) => write!(f, "{string}"),
        }
    }
}

impl CompTimeSize<'_> for Expr<'_> {
    fn number_bytes(&self, ctx: &ProgramContext) -> ByteSize {
        match self {
            Self::FnDef(_) => unreachable!(),
            Self::FnCall(_) => todo!(), //fn_call.number_bytes(),
            Self::Type(_) => ByteSize::Exact(0),
            Self::Assign(_) => todo!(), //assign.number_bytes(),
            Self::SmallValue(value) => value.number_bytes(ctx),
            Self::Bytes(bytes) => ByteSize::Exact(bytes.len()),
            Self::StringSlice(slice) => ByteSize::Exact(slice.bytes().len()),
        }
    }
}

impl<'source> AstNode<'source> for Expr<'source> {
    fn build_context(&self, ctx: &mut ProgramContext<'source>, scope_stack: &mut Vec<ScopeId>) {
        match self {
            Expr::FnDef(fn_def) => fn_def.build_context(ctx, scope_stack),
            Expr::Assign(assign) => assign.build_context(ctx, scope_stack),
            Expr::Type(_)
            | Expr::FnCall(_)
            | Expr::SmallValue(_)
            | Expr::Bytes(_)
            | Expr::StringSlice(_) => {}
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
            Expr::Type(type_) => write!(output, "{}Type({:?})", current_padding(), type_)?,
            Expr::StringSlice(string) => write!(output, "{}String({})", current_padding(), string)?,
            Expr::Bytes(bytes) => write!(output, "{}Bytes({:?})", current_padding(), bytes)?,
            Expr::SmallValue(value) => write!(output, "{}Value({})", current_padding(), value)?,
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
                Ok(Expr::SmallValue(SmallValue::DWord(val)))
            }
            Token::String => {
                let strval = parser.current_slice;
                parser.advance();
                Ok(Expr::StringSlice(strval))
            }
            Token::KeywordFn => {
                parser.advance();
                Ok(Expr::FnDef(FnDef::parse(parser)?))
            }
            Token::KeywordType => {
                parser.advance();
                Ok(Expr::Type(typeexpr::Type::parse(parser)?))
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
