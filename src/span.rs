use std::ops::{Index, Range};

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[inline]
    pub fn empty() -> Self {
        Self { start: 0, end: 0 }
    }

    #[inline]
    pub fn start(self) -> usize {
        self.start as usize
    }

    #[inline]
    pub fn end(self) -> usize {
        self.end as usize
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start as u32,
            end: value.end as u32,
        }
    }
}

impl From<Span> for std::ops::Range<usize> {
    fn from(value: Span) -> Self {
        Self {
            start: value.start as usize,
            end: value.end as usize,
        }
    }
}

impl Index<Span> for str {
    type Output = <str as Index<Range<usize>>>::Output;
    fn index(&self, index: Span) -> &Self::Output {
        self.index(Range::from(index))
    }
}
