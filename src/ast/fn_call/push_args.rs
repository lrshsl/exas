use std::io;

use super::*;

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
        .filter_map(|(param, arg)| match &param {
            Param::LiteralMatcher(_) => None,
            Param::ParamExpr(ParamExpr { typename, .. }) => {
                let Some(typename) = typename else {
                    todo!("Type inference and generics not yet implemented");
                };
                let type_ = find_type(ctx, typename).unwrap();
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
