use super::*;

pub fn resolve_arg_size(
    ctx: &ProgramContext,
    type_: &typeexpr::Type,
    arg: &RawToken,
) -> Result<usize, CheckError> {
    let type_size = &type_.size;
    let arg_size = &arg.number_bytes(ctx);

    match type_size.overlap(arg_size) {
        None => compile_error(
            ctx.file_context.clone(),
            format!("Type size mismatch: no overlap between {type_size:?} and {arg_size:?}")
                .to_string(),
        ),
        Some(ByteSize::Exact(size)) => Ok(size),
        Some(ByteSize::Range(_)) => todo!("Default size for type?"),
        Some(ByteSize::AnySize) => unreachable!("wtf should I do here"),
    }
}
