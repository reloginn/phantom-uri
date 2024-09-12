#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    pub const fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }
    pub const fn start(&self) -> usize {
        self.start
    }
    pub const fn length(&self) -> usize {
        self.length
    }
}
