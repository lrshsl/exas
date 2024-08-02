use crate::{
    ast::{
        current_padding, current_scope, next_scope, Assign, FnCall, FnDef, Ident, Int, ScopeId,
        Symbol,
    },
    parser::ParsingError,
    AstNode, ProgramContext,
};

#[derive(Clone)]
pub enum Expr {
    FnDef(FnDef),
    FnCall(FnCall),

    Assign(Assign),

    Ident(Ident),
    Int(Int),
    String(&'static str),
}

impl AstNode for Expr {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        match self {
            Expr::FnDef(fn_def) => fn_def.build_context(ctx, current_scope),
            Expr::Assign(assign) => assign.build_context(ctx, current_scope),
            Expr::Ident(ident) => ident.build_context(ctx, current_scope),

            Expr::Int(_) => {}
            Expr::String(_) => {}
            Expr::FnCall(_) => {}
        }
    }

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        match self {
            Expr::FnDef(fn_def) => fn_def.emit(ctx, scope_stack),
            Expr::Ident(ident) => {
                // Check if in scope
                if let Some(name_matches) = ctx.symbols.get(ident.0) {
                    let scope_matches = name_matches.iter().filter(|symbol| {
                        scope_stack.iter().any(|scope_id| scope_id == &symbol.scope)
                    });
                    match scope_matches.count() {
                        0 => panic!(
                            "<{}> not defined in this scope (scope {})",
                            ident.0,
                            scope_stack.last().unwrap()
                        ),
                        1 => ident.emit(ctx, scope_stack),
                        _ => panic!(
                            "<{}> defined multiple times in this scope (scope {})",
                            ident.0,
                            scope_stack.last().unwrap()
                        ),
                    }
                } else {
                    panic!(
                        "<{}> not defined in this scope (scope {})",
                        ident.0,
                        scope_stack.last().unwrap()
                    );
                }
            }

            Expr::Assign(assign) => assign.emit(ctx, scope_stack),
            Expr::Int(int) => println!("{}Int({})", current_padding(), int.0),
            Expr::String(string) => println!("{}String({})", current_padding(), string),
            Expr::FnCall(fn_call) => fn_call.emit(ctx, scope_stack),
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FnDef(fn_def) => write!(f, "{:?}", fn_def),
            Expr::FnCall(fn_call) => write!(f, "{:?}", fn_call),
            Expr::Assign(assign) => write!(f, "{:?}", assign),
            Expr::Ident(ident) => write!(f, "{:?}", ident),
            Expr::Int(int) => write!(f, "{:?}", int),
            Expr::String(string) => write!(f, "{:?}", string),
        }
    }
}
