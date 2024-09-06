use super::*;

pub(super) fn stack_pop_remaining_parameters<Output: std::io::Write>(
    ctx: &ProgramContext,
    output: &mut Output,
    params: &Vec<Param>,
) -> CheckResult<()> {
    for (i, param) in params
        .iter()
        .filter_map(|p| match p {
            // Ignore literals
            Param::ParamExpr(p) => Some(p),
            Param::LiteralMatcher(_) => None,
        })
        .enumerate()
    {
        match param.number_bytes(ctx) {
            // todo
            _ => {
                write!(
                    output,
                    "{pad}pop {size} -> {reg}       | {i}th argument",
                    pad = current_padding(),
                    size = param.number_bytes(ctx),
                    reg = free_register()
                )?;
                if let Some(name) = param.name {
                    write!(output, ": {name}")?;
                }
                writeln!(output)?;
            }
        }
    }
    Ok(())
}
