
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    pub offset: usize,
    pub length: usize,
}

impl Span {
    pub fn new(offset: usize, length: usize) -> Self {
        Self { offset, length }
    }
}