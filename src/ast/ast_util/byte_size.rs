use std::{fmt, ops};

#[derive(Debug, Clone, PartialEq)]
pub enum ByteSize {
    Exact(usize),
    Range(ops::Range<usize>),
    AnySize,
}

impl fmt::Display for ByteSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteSize::Exact(val) => write!(f, "{}b", val),
            ByteSize::Range(ops::Range { start, end }) => {
                write!(f, "({start}..{end})b")
            }
            ByteSize::AnySize => write!(f, "AnySize"),
        }
    }
}

impl ByteSize {
    pub fn overlap(&self, other: &ByteSize) -> Option<ByteSize> {
        match self {
            ByteSize::AnySize => Some(other.clone()),
            ByteSize::Exact(size) => match other {
                // 4 <-> 4 => 4
                ByteSize::Exact(other_size) if other_size == size => Some(ByteSize::Exact(*size)),

                // 4 <-> 1..8 => 4
                ByteSize::Range(other_range) if other_range.contains(&size) => {
                    Some(ByteSize::Exact(*size))
                }

                ByteSize::AnySize => Some(ByteSize::AnySize),
                // 4 <-> 1..2 => None
                _ => None,
            },
            ByteSize::Range(ref self_range) => match other {
                // 1..8 <-> 4 => 4
                ByteSize::Exact(other_size) if self_range.contains(&other_size) => {
                    Some(ByteSize::Exact(*other_size))
                }
                // 1..4 <-> 2..8 => 2..4
                ByteSize::Range(other_range) => Some(ByteSize::Range(
                    self_range.start.max(other_range.start)..self_range.end.min(other_range.end),
                )),

                // 1..2 <-> 4
                // 1..2 <-> 4..8
                _ => None,
            },
        }
    }
}
