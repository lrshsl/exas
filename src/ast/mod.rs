use std::{
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::lexer::Token;

mod expr;

pub use expr::Expr;


pub trait AstNode {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId);
    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>);
}

static HIGHEST_SCOPE: AtomicUsize = AtomicUsize::new(1);
static INDENTATION_LEVEL: AtomicUsize = AtomicUsize::new(0);

fn current_padding() -> String {
    " ".repeat(INDENTATION_LEVEL.load(Ordering::Relaxed) * 4)
}

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
            println!("{:?}", stmt);
        }
    }
}

impl AstNode for Ast {
    fn build_context(&self, ctx: &mut ProgramContext, _: ScopeId) {
        self.stmts.build_context(ctx, 0);
    }

    fn emit(&self, ctx: &ProgramContext, _: &mut Vec<ScopeId>) {
        let mut scope_stack = Vec::from([0]);
        self.stmts.emit(ctx, &mut scope_stack);
    }
}

#[derive(Debug, Clone)]
pub struct ListContent {
    pub elements: Vec<Expr>,
}

impl AstNode for ListContent {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        let new_scope = next_scope();

        for element in self.elements.iter() {
            element.build_context(ctx, new_scope);
        }
    }

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<usize>) {
        // Start a new scope
        println!("{}{{", current_padding());
        scope_stack.push(next_scope());
        INDENTATION_LEVEL.fetch_add(1, Ordering::Relaxed);

        for element in &self.elements {
            element.emit(ctx, scope_stack);
        }

        INDENTATION_LEVEL.fetch_sub(1, Ordering::Relaxed);
        scope_stack.pop();
        println!("{}}}", current_padding());
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
                1 => println!("{}Int({})", current_padding(), self.0),
                _ => panic!("<{}> defined multiple times in this scope", self.0),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Int(pub i32);

#[derive(Clone, Debug)]
pub struct Assign {
    pub name: &'static str,
    pub value: Rc<Expr>,
}

impl AstNode for Assign {
    fn build_context(&self, ctx: &mut ProgramContext, current_scope: ScopeId) {
        let entry = ctx.symbols.entry(self.name).or_insert(Vec::new());

        if entry.iter().any(|symbol| symbol.scope == current_scope) {
            panic!(
                "Assignment of <{}> shadows other variable in scope",
                self.name
            );
        }
        entry.push(Symbol {
            scope: current_scope,
            value: Expr::Assign(self.clone()),
        });
    }

    fn emit(&self, ctx: &ProgramContext, scope_stack: &mut Vec<ScopeId>) {
        println!("{}let {} = ", current_padding(), self.name);
        self.value.emit(ctx, scope_stack);
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
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: &'static str,
    pub pattern: MatchPattern,
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
        println!("{}fn {:?}", current_padding(), self.signature.params);
        self.body.emit(ctx, scope_stack);
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
        println!(
            "{}FnCall({:?}, {:?})",
            current_padding(),
            self.fn_name,
            self.args
        );
    }
}

impl std::fmt::Debug for FnCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnCall({:?}, {:?})", self.fn_name, self.args)
    }
}
