use super::*;

pub(crate) trait CompTimeSize<'source> {
    fn number_bytes(&self, ctx: &'source ProgramContext) -> ByteSize;
}
