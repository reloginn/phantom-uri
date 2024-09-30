#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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
    pub const fn end(&self) -> usize {
        self.start() + self.length()
    }
    pub fn add_to_length(&mut self, num: usize) {
        self.length += num
    }
    pub fn set_start(&mut self, num: usize) {
        self.start = num;
    }
}
