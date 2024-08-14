use expr::Expr;

use super::*;

use std::{
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

static HIGHEST_SCOPE: AtomicUsize = AtomicUsize::new(0);
static INDENTATION_LEVEL: AtomicUsize = AtomicUsize::new(0);

pub enum IndentationChange {
    More,
    Less,
}

pub(super) fn current_padding() -> String {
    " ".repeat(INDENTATION_LEVEL.load(Ordering::Relaxed) * 4)
}

pub(super) fn next_scope() -> usize {
    HIGHEST_SCOPE.fetch_add(1, Ordering::Relaxed)
}

pub(super) fn reset_scope_and_indent() {
    HIGHEST_SCOPE.store(0, Ordering::Relaxed);
    INDENTATION_LEVEL.store(0, Ordering::Relaxed)
}

pub(super) fn change_indentation(change: IndentationChange) -> usize {
    match change {
        IndentationChange::More => INDENTATION_LEVEL.fetch_add(1, Ordering::Relaxed),
        IndentationChange::Less => INDENTATION_LEVEL.fetch_sub(1, Ordering::Relaxed),
    }
}

pub(super) type ScopeId = usize;

#[derive(Debug)]
pub struct ProgramContext {
    pub symbols: SymbolTable,
    pub file_context: FileContext,
}

pub type SymbolTable = HashMap<&'static str, Vec<Symbol>>;

#[derive(Debug)]
pub struct Symbol {
    pub scope: ScopeId,
    pub value: Rc<Expr>,
}
