use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::lexer::Token;

pub trait AstNode {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId);
    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>);
}

static HIGHEST_SCOPE: AtomicUsize = AtomicUsize::new(1);
static INDENTATION_LEVEL: AtomicUsize = AtomicUsize::new(1);

fn current_scope() -> usize {
    HIGHEST_SCOPE.load(Ordering::Relaxed)
}

fn next_scope() -> usize {
    HIGHEST_SCOPE.fetch_add(1, Ordering::Relaxed)
}

pub type ScopeId = usize;

pub struct ProgramContext {
    pub symbols: SymbolTable,
}

pub type SymbolTable = HashMap<&'static str, Vec<Symbol>>;

pub struct Symbol {
    pub scope: ScopeId,
    pub value: Expr,
}

#[derive(Debug)]
pub struct Ast {
    pub stmts: ListContent,
}

impl Ast {
    pub fn print(&self) {
        for stmt in &self.stmts.elements {
            println!("{:?}\n", stmt);
        }
    }
}

impl AstNode for Ast {
    fn build_context(&self, ctx: &mut ProgramContext, _: ScopeId) {
        for stmt in self.stmts.elements.iter() {
            stmt.build_context(ctx, 0);
        }
    }

    fn emit(&self, ctx: &ProgramContext, _: &mut Vec<ScopeId>) {
        let mut scope_stack = Vec::from([0]);
        println!("{{");

        for stmt in &self.stmts.elements {
            stmt.emit(ctx, &mut scope_stack);
        }

        println!("}}");
    }
}

#[derive(Debug, Clone)]
pub struct ListContent {
    pub elements: Vec<Expr>,
}

impl AstNode for ListContent {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        for element in self.elements.iter() {
            element.build_context(ctx, current_scope);
        }
    }

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<usize>) {
        // Start a new scope
        scope_stack.push(next_scope());

        for element in &self.elements {
            element.emit(ctx, scope_stack);
        }

        scope_stack.pop();
    }
}

#[derive(Debug, Clone)]
pub struct Ident(pub &'static str);

impl AstNode for Ident {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        ctx.symbols
            .entry(self.0)
            .or_insert(Vec::new())
            .push(Symbol {
                scope: current_scope,
                value: Expr::Ident(self.clone()),
            });
    }

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        if let Some(name_matches) = ctx.symbols.get(self.0) {
            let scope_matches = name_matches
                .iter()
                .filter(|symbol| scope_stack.iter().any(|scope_id| scope_id == &symbol.scope));
            match scope_matches.count() {
                0 => panic!("<{}> not defined in this scope", self.0),
                1 => println!("Int({})", self.0),
                _ => panic!("<{}> defined multiple times in this scope", self.0),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Int(pub i32);

#[derive(Clone)]
pub enum Expr {
    FnDef(FnDef),
    FnCall(FnCall),

    Ident(Ident),
    Int(Int),
    String(&'static str),
}

impl AstNode for Expr {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        match self {
            Expr::FnDef(fn_def) => {
                let entry = ctx
                    .symbols
                    .entry(fn_def.signature.name)
                    .or_insert(Vec::new());

                if entry.iter().any(|symbol| symbol.scope == current_scope) {
                    panic!(
                        "Expr {} already defined in this scope, fn overloading not yet supported",
                        fn_def.signature.name
                    );
                }
                entry.push(Symbol {
                    scope: current_scope,
                    value: Expr::FnDef(fn_def.clone()),
                });

                fn_def.build_context(ctx, next_scope());
            }
            Expr::Ident(ident) => ident.build_context(ctx, current_scope),

            Expr::Int(_) => {}
            Expr::String(_) => {}
            Expr::FnCall(_) => {}
        }
    }

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        match self {
            Expr::FnDef(fn_def) => {
                if let Some(name_matches) = ctx.symbols.get(fn_def.signature.name) {
                    let scope_matches = name_matches.iter().filter(|symbol| {
                        // TODO: Check if parameters match
                        scope_stack.iter().any(|scope_id| scope_id == &symbol.scope)
                    });
                    match scope_matches.count() {
                        0 => panic!("Function <{}> not accessable in this scope (scope {})",
                            fn_def.signature.name, scope_stack.last().unwrap()),
                        1 => {}
                        _ => panic!("Function <{}> defined multiple times, fn overloading not yet supported",
                            fn_def.signature.name),
                    }
                    fn_def.emit(ctx, scope_stack);
                } else {
                    panic!("Function <{}> not defined", fn_def.signature.name,);
                }
            }
            Expr::Ident(ident) => {
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
                        1 => {}
                        _ => panic!(
                            "<{}> defined multiple times in this scope (scope {})",
                            ident.0,
                            scope_stack.last().unwrap()
                        ),
                    }
                    ident.emit(ctx, scope_stack);
                } else {
                    panic!(
                        "<{}> not defined in this scope (scope {})",
                        ident.0,
                        scope_stack.last().unwrap()
                    );
                }
            }

            Expr::Int(int) => println!("{}", int.0),
            Expr::String(string) => println!("{}", string),
            Expr::FnCall(fn_call) => fn_call.emit(ctx, scope_stack),
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::FnDef(fn_def) => write!(f, "{:?}", fn_def),
            Expr::FnCall(fn_call) => write!(f, "{:?}", fn_call),
            Expr::Ident(ident) => write!(f, "{:?}", ident),
            Expr::Int(int) => write!(f, "{:?}", int),
            Expr::String(string) => write!(f, "{:?}", string),
        }
    }
}

#[derive(Clone)]
pub enum RawToken {
    Ident(Ident),
    Int(Int),
    String(&'static str),
    Symbol(char),

    Expr(Expr),
}

impl std::fmt::Debug for RawToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawToken::Ident(ident) => write!(f, "{:?}", ident),
            RawToken::Int(int) => write!(f, "{:?}", int),
            RawToken::String(string) => write!(f, "{:?}", string),
            RawToken::Symbol(symbol) => write!(f, "{:?}", symbol),
            RawToken::Expr(expr) => write!(f, "{:?}", expr),
        }
    }
}

impl RawToken {
    pub fn from_token(token: Token, slice: &'static str) -> Self {
        match token {
            Token::Ident => RawToken::Ident(Ident(slice)),
            Token::Int => RawToken::Int(Int(slice.parse().expect("Should be able to parse int"))),
            Token::String => RawToken::String(slice),
            _ => {
                assert_eq!(slice.len(), 1);
                RawToken::Symbol(slice.chars().next().unwrap())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FnSignature {
    pub name: &'static str,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
struct Param {
    name: &'static str,
    pattern: MatchPattern,
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    RawToken(RawToken),
    TypeExpr,
}

#[derive(Debug, Clone)]
pub struct FnDef {
    pub signature: FnSignature,
    pub body: ListContent,
}

impl AstNode for FnDef {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: usize) {
        self.body.build_context(ctx, next_scope());
    }

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        println!(
            "Fn(name = {}, params = {:?}, scope: {}) {{",
            self.signature.name,
            self.signature.params,
            scope_stack.last().unwrap()
        );
        self.body.emit(ctx, scope_stack);
        println!("}} // end fn {}", self.signature.name);
    }
}

#[derive(Clone)]
pub struct FnCall {
    pub fn_name: &'static str,
    pub args: Vec<RawToken>,
}

impl AstNode for FnCall {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {}

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        println!("FnCall({:?}, {:?})", self.fn_name, self.args);
    }
}

impl std::fmt::Debug for FnCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnCall({:?}, {:?})", self.fn_name, self.args)
    }
}
