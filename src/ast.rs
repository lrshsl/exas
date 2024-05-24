
pub struct Ast {
    pub stmts: Vec<Statement>,
}

impl Ast {
    pub fn print(&self) {
        for stmt in &self.stmts {
            println!("{}", stmt);
        }
    }
}

pub enum Statement {
    Expr(Expr),
    Set(String, Expr),
    Print(Expr),
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::Expr(expr) => write!(f, "{}", expr),
            Statement::Set(ident, expr) => write!(f, "set {} = {}", ident, expr),
            Statement::Print(expr) => write!(f, "print {}", expr),
        }
    }
}

pub enum Expr {
    Ident(String),
    Int(i32),
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Ident(ident) => write!(f, "{}", ident),
            Expr::Int(int) => write!(f, "{}", int),
        }
    }
}

