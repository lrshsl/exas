use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FnSignature<'source> {
    pub params: ParamList<'source>,
}

impl FnSignature<'_> {
    pub fn matches_args(&self, ctx: &ProgramContext, args: &Vec<RawToken>) -> bool {
        self.params.len() == args.len()
            && self
                .params
                .iter()
                .zip(args.iter())
                .all(|(param, arg)| param.matches_arg(ctx, arg))
    }
}
