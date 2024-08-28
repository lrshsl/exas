use expr::SmallValue;
use params::MatchPattern;
use typeexpr::find_type;

use super::*;
use crate::errors::compile_error;

pub fn resolve_arg_size(
    ctx: &ProgramContext,
    type_: &typeexpr::Type,
    arg: &RawToken,
) -> Result<usize, CheckError> {
    match type_.size {
        ByteSize::Exact(size) => Ok(size),
        ByteSize::Range(ref param_range) => {
            match arg.number_bytes(ctx) {
                ByteSize::Exact(arg_size) if param_range.contains(&arg_size) => Ok(arg_size),
                ByteSize::Range(arg_range) => {
                    // Return smallest possible
                    let first_common_point = param_range.start.max(arg_range.start);
                    if param_range.contains(&first_common_point)
                        && arg_range.contains(&first_common_point)
                    {
                        Ok(first_common_point)
                    } else {
                        compile_error(ctx.file_context.clone(), format!("Type size mismatch: no overlap between {arg_range:?} and {param_range:?}").to_string())
                    }
                }
                // TODO: Makes sense?
                ByteSize::Exact(arg_size) => compile_error(
                    ctx.file_context.clone(),
                    format!("Type size mismatch: {arg_size}b not in {param_range:?}").to_string(),
                ),
            }
        }
    }
}

pub fn push_args<Output: io::Write>(
    output: &mut Output,
    ctx: &ProgramContext,
    args: &[RawToken],
    params: &[Param],
) -> CheckResult<()> {
    let number_bytes = params
        .iter()
        .zip(args)
        .rev()
        .map(|(param, arg)| match &param.pattern {
            MatchPattern::RawToken(token) => {
                let ByteSize::Exact(size) = token.number_bytes(ctx) else {
                    unreachable!()
                };
                Ok(size)
            }
            MatchPattern::TypeExpr { typename } => {
                let type_ = find_type(ctx, *typename).unwrap();
                resolve_arg_size(ctx, type_, arg)
            }
        });
    // TODO: Properly handle errors
    assert!(!number_bytes.clone().any(|e| e.is_err()));
    assert_eq!(number_bytes.len(), args.len());
    for (number_bytes, arg) in number_bytes.flatten().zip(args.iter().rev()) {
        writeln!(output, "push {}b {:?}", number_bytes, arg)?;
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall<'source> {
    pub name: &'source str,
    pub args: Vec<RawToken<'source>>,
}

impl AstNode<'_> for FnCall<'_> {
    fn build_context(&self, _ctx: &mut ProgramContext, _scope_stack: &mut Vec<ScopeId>) {}

    fn check_and_emit<Output: std::io::Write>(
        &self,
        output: &mut Output,
        ctx: &ProgramContext<'_>,
        scope_stack: &mut Vec<ScopeId>,
    ) -> CheckResult<()> {
        let function_entry = match ctx.symbols.get(self.name) {
            None => {
                return compile_error(
                    ctx.file_context.clone(),
                    format!("Undefined function: {}", self.name).to_string(),
                )
            }
            Some(global_matches) => match global_matches
                .iter()
                .filter(|f| scope_stack.contains(&f.scope))
                .collect::<Vec<_>>()[..]
            {
                [one_match] => one_match,
                [] => {
                    return compile_error(
                        ctx.file_context.clone(),
                        format!(
                            "Function not defined in this scope: {}, scope: {}",
                            self.name,
                            scope_stack.last().unwrap()
                        ),
                    )
                }
                [..] => {
                    return compile_error(
                        ctx.file_context.clone(),
                        format!(
                            "Function defined multiple times in this scope: {}",
                            self.name
                        ),
                    )
                }
            },
        };
        // Hope it's a function
        let Expr::FnDef(fn_def) = function_entry.value.as_ref() else {
            todo!("Found only a variable.. Probably should ignore those")
        };
        // Check signature
        if !fn_def.signature.matches_args(ctx, &self.args) {
            return compile_error(
                ctx.file_context.clone(),
                format!(
                    "Function signature mismatch: {}\nArgs {actual:?} don't match. Best candidate is {expected:?}",
                    self.name,
                    actual = self.args,
                    expected = fn_def.signature
                )
                .to_string(),
            );
        }
        // How do I find the number of bytes?
        // Either the args or the signature have the specific size
        //
        // f = fn [:Number] {},     | Fn def generic
        // arg 1b = 5
        // f arg,
        //
        // f = fn [:u8]
        // f 7,                     | Fn call generic
        //
        writeln!(
            output,
            "\n{pad}| Function call: {name}",
            name = self.name,
            pad = current_padding()
        )?;
        push_args(output, ctx, &self.args, &fn_def.signature.params)?;
        writeln!(output, "{}call {}", current_padding(), self.name)?;
        Ok(())
    }
}

pub type ArgumentList<'source> = Vec<RawToken<'source>>;

impl<'source> Parsable<'source> for ArgumentList<'source> {
    fn parse(parser: &mut Parser<'source>) -> Result<ArgumentList<'source>, ParsingError<'source>> {
        let mut args = vec![];
        while let Some(Ok(token)) = parser.current_token.as_ref() {
            match token {
                Token::Symbol(")") | Token::Symbol(",") => {
                    parser.advance();
                    break;
                }
                Token::Symbol("(") => {
                    parser.advance();
                    args.push(RawToken::Expr(Expr::parse(parser)?))
                }
                token => {
                    args.push(RawToken::from_token(token, parser.current_slice));
                    parser.advance();
                } // TODO: Some other tokens are not allowed here?
            }
        }
        Ok(args)
    }
}
