
use crate::lexer::Token;

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

#[derive(Debug)]
pub struct ListContent {
    pub elements: Vec<Expr>,
}

#[derive(Debug)]
pub struct Ident(pub &'static str);

#[derive(Debug)]
pub struct Int(pub i32);

#[derive(Debug)]
pub struct TokString(pub &'static str);

pub enum Expr {
    FnDef(FnDef),
    FnCall(FnCall),

    Ident(Ident),
    Int(Int),
    String(TokString),
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

pub enum RawToken {
    Ident(Ident),
    Int(Int),
    String(TokString),
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
            Token::String => RawToken::String(TokString(slice)),
            _ => {
                assert_eq!(slice.len(), 1);
                RawToken::Symbol(slice.chars().next().unwrap())
            }
        }
    }
}

#[derive(Debug)]
pub struct FnDef {
    pub fn_name: TokString,
    pub params: Vec<TokString>,
    pub body: ListContent,
}

pub struct FnCall {
    pub fn_name: &'static str,
    pub args: Vec<RawToken>,
}

impl std::fmt::Debug for FnCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FnCall({:?}, {:?})", self.fn_name, self.args)
    }
}
