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
    let number_bytes =
        params
            .iter()
            .zip(args)
            .rev()
            .filter_map(|(param, arg)| match &param.pattern {
                MatchPattern::RawToken(RawToken::Ident(_)) => None,
                MatchPattern::RawToken(token) => {
                    let ByteSize::Exact(size) = token.number_bytes(ctx) else {
                        unreachable!()
                    };
                    Some(Ok((size, arg)))
                }
                MatchPattern::TypeExpr { typename } => {
                    let type_ = find_type(ctx, *typename).unwrap();
                    let size = match resolve_arg_size(ctx, type_, arg) {
                        Ok(size) => size,
                        Err(err) => return Some(Err(err)),
                    };
                    Some(Ok((size, arg)))
                }
            });
    // TODO: Properly handle errors
    for e in number_bytes {
        let Ok((size, arg)) = e else {
            return Err(e.unwrap_err());
        };
        writeln!(output, "push {}b {:?}", size, arg)?;
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
        // Find all symbols with that name
        let Some(global_matches) = ctx.symbols.get(self.name) else {
            return compile_error(
                ctx.file_context.clone(),
                format!("Function not found anywhere: {}", self.name).to_string(),
            );
        };
        // Filter out functions that are not in scope
        let scope_matches = global_matches
            .iter()
            .filter(|f| scope_stack.contains(&f.scope));
        if scope_matches.clone().next().is_none() {
            return compile_error(
                ctx.file_context.clone(),
                format!(
                    "Function not defined in this scope: {}, scope: {}",
                    self.name,
                    scope_stack.last().unwrap()
                ),
            );
        }
        // Retain only functions
        let function_matches = scope_matches.filter_map(|f| match f.value.as_ref() {
            Expr::FnDef(fn_def) => Some(fn_def),
            _ => None,
        });
        let Some(first_fn_match) = function_matches.clone().next() else {
            return compile_error(
                ctx.file_context.clone(),
                format!(
                    "Function not found {name}: {name} exists in this scope, but is not callable",
                    name = self.name
                )
                .to_string(),
            );
        };
        // Check signature
        let mut signature_matches =
            function_matches.filter(|f| f.signature.matches_args(ctx, &self.args));
        let Some(fn_def) = signature_matches.next() else {
            return compile_error(
                ctx.file_context.clone(),
                format!(
                    "Function signature mismatch: \"{name}\"\nArgs {actual:#?} don't match to any signature.\n\nNote: One candidate is \"{name}\" with signature:\n{expected:#?}",
                    name = self.name,
                    actual = self.args,
                    expected = first_fn_match.signature
                )
                .to_string(),
            );
        };
        // Should only have one match
        if signature_matches.next().is_some() {
            return compile_error(
                ctx.file_context.clone(),
                format!(
                    "Found two matching functions for {name} with the given arguments. Consider adding a ident to the function signature to distinguish them.",
                    name = self.name,
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
