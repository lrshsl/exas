pub use super::*;

pub fn find_type<'source>(ctx: &'source ProgramContext, typename: &str) -> Option<&'source Type> {
    ctx.types.get(typename)
}
