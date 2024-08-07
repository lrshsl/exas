use expr::Expr;

use super::*;

use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

static HIGHEST_SCOPE: AtomicUsize = AtomicUsize::new(1);
static INDENTATION_LEVEL: AtomicUsize = AtomicUsize::new(0);

pub enum IndentationChange {
    More,
    Less,
}

pub(super) fn current_padding() -> String {
    " ".repeat(INDENTATION_LEVEL.load(Ordering::Relaxed) * 4)
}

pub(super) fn current_scope() -> usize {
    HIGHEST_SCOPE.load(Ordering::Relaxed)
}

pub(super) fn next_scope() -> usize {
    HIGHEST_SCOPE.fetch_add(1, Ordering::Relaxed)
}

pub(super) fn change_indentation(change: IndentationChange) {
    match change {
        IndentationChange::More => INDENTATION_LEVEL.fetch_add(1, Ordering::Relaxed),
        IndentationChange::Less => INDENTATION_LEVEL.fetch_sub(1, Ordering::Relaxed),
    };
}

pub(super) type ScopeId = usize;

pub struct ProgramContext {
    pub symbols: SymbolTable,
}

pub type SymbolTable = HashMap<&'static str, Vec<Symbol>>;

#[derive(Debug)]
pub struct Symbol {
    pub scope: ScopeId,
    pub value: Expr,
}
