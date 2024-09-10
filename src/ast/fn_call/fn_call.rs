use super::*;
use crate::errors::compile_error;

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
                    "Function signature mismatch: \"{name}\"\nArgs {actual:#?} don't match to any \
                     signature.\n\nNote: One candidate is \"{name}\" with \
                     signature:\n{expected:#?}",
                    name = self.name,
                    actual = self.args,
                    expected = first_fn_match.signature
                )
                .to_string(),
            );
        };
        // Should only have one match
        if let Some(second_fn_def) = signature_matches.next() {
            return compile_error(
                ctx.file_context.clone(),
                format!(
                    "Found two matching functions for \"{name}\" with the given \
                     arguments:\n\nArguments: {args:?}\n\n| The following function definitions \
                     match:\n{fn_sig1:#?}\n| as well as:\n{fn_sig2:#?}\n| Consider adding a ident \
                     to the function signature to distinguish them.",
                    name = self.name,
                    args = self.args,
                    fn_sig1 = fn_def.signature,
                    fn_sig2 = second_fn_def.signature,
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
        // @see ByteSize::overlap(..)
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
