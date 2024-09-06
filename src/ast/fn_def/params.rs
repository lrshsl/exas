use super::*;

#[derive(Clone, PartialEq)]
pub enum Param<'source> {
    LiteralMatcher(RawToken<'source>),
    ParamExpr(ParamExpr<'source>),
}

impl Param<'_> {
    pub fn matches_arg(&self, ctx: &ProgramContext<'_>, arg: &RawToken<'_>) -> bool {
        match self {
            Param::LiteralMatcher(raw_token) => arg == raw_token,
            Param::ParamExpr(param_expr) => param_expr.matches_arg(ctx, arg),
        }
    }
}

impl<'source> CompTimeSize<'source> for Param<'source> {
    fn number_bytes(&self, ctx: &'source ProgramContext) -> ByteSize {
        match &self {
            Param::LiteralMatcher(raw_token) => raw_token.number_bytes(ctx),
            Param::ParamExpr(param_expr) => param_expr.number_bytes(ctx),
        }
    }
}

impl std::fmt::Debug for Param<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Param::LiteralMatcher(raw_token) => write!(f, "LiteralMatcher<{:?}>", raw_token),
            Param::ParamExpr(param_expr) => write!(f, "ParamExpr<{:?}>", param_expr),
        }
    }
}
