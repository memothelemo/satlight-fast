use satlight_macros::Getter;
use std::ops::Range;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Getter, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Span {
    #[clone]
    pub(crate) start: usize,
    #[clone]
    pub(crate) end: usize,
}

impl Span {
    pub const DUMMY: Span = Span { start: 0, end: 0 };

    pub const fn new(start: usize, end: usize) -> Self {
        Self {
            start: start + 1,
            end: end + 1,
        }
    }

    #[inline(always)]
    pub const fn top() -> Self {
        Self { start: 1, end: 1 }
    }
}

impl Span {
    pub fn is_dummy(&self) -> bool {
        self.start == 0 && self.end == 0
    }

    pub fn is_valid(&self) -> bool {
        self.start <= self.end
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            start: usize::min(self.start, other.start),
            end: usize::max(self.end, other.end),
        }
    }

    pub fn range(&mut self) -> Range<usize> {
        Range {
            start: self.start - 1,
            end: self.end - 1,
        }
    }

    pub fn with_start(&self, start: usize) -> Self {
        Self {
            start: start + 1,
            end: self.end,
        }
    }

    pub fn with_end(&self, end: usize) -> Self {
        Self {
            start: self.start,
            end: end + 1,
        }
    }
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_dummy() {
            write!(f, "Span(-1,-1)")
        } else {
            write!(f, "Span({},{})", self.start - 1, self.end - 1)
        }
    }
}
