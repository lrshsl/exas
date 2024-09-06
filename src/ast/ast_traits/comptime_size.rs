use super::*;

pub trait CompTimeSize<'source> {
    fn number_bytes(&self, ctx: &'source ProgramContext) -> ByteSize;
}
