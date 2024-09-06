use std::{
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use super::*;

static HIGHEST_SCOPE: AtomicUsize = AtomicUsize::new(0);
static INDENTATION_LEVEL: AtomicUsize = AtomicUsize::new(0);

pub enum IndentationChange {
    More,
    Less,
}

pub fn current_padding() -> String {
    " ".repeat(INDENTATION_LEVEL.load(Ordering::Relaxed) * 4)
}

pub fn next_scope() -> usize {
    HIGHEST_SCOPE.fetch_add(1, Ordering::Relaxed)
}

pub fn reset_scope_and_indent() {
    HIGHEST_SCOPE.store(0, Ordering::Relaxed);
    INDENTATION_LEVEL.store(0, Ordering::Relaxed)
}

pub fn change_indentation(change: IndentationChange) -> usize {
    match change {
        IndentationChange::More => INDENTATION_LEVEL.fetch_add(1, Ordering::Relaxed),
        IndentationChange::Less => INDENTATION_LEVEL.fetch_sub(1, Ordering::Relaxed),
    }
}

pub type ScopeId = usize;

#[derive(Debug)]
pub struct ProgramContext<'source> {
    pub symbols: SymbolTable<'source>,
    pub types: HashMap<&'source str, typeexpr::Type>,
    pub file_context: FileContext<'source>,
}

pub type SymbolTable<'source> = HashMap<&'source str, Vec<Symbol<'source>>>;

#[derive(Debug)]
pub struct Symbol<'source> {
    pub scope: ScopeId,
    pub value: Rc<Expr<'source>>,
}
