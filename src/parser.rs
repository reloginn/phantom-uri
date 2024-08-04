use std::ops::AddAssign;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum State {
    Start,
    Continue,
    Eof,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Parser<'a> {
    slice: Slice<'a>,
    position: usize,
    state: State,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            slice: Slice::new(data),
            position: 0,
            state: State::Start,
        }
    }
    pub fn next(&mut self) {
        if self.position >= self.eof() {
            self.state = State::Eof;
            return;
        }
        self.position.add_assign(1);
        self.state = State::Continue
    }
    pub fn get_byte(&self) -> u8 {
        let &byte = unsafe { self.get_unchecked(self.position) };
        byte
    }
    pub fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.next()
        }
    }
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: std::slice::SliceIndex<[u8]>,
    {
        self.slice.get_unchecked(index)
    }
    pub const fn eof(&self) -> usize {
        self.slice.len().wrapping_sub(1)
    }
    pub const fn current_position(&self) -> usize {
        self.position
    }
    pub const fn state(&self) -> State {
        self.state
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Slice<'a> {
    data: &'a [u8],
}

impl<'a> Slice<'a> {
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
    pub unsafe fn get_unchecked<I>(&self, index: I) -> &I::Output
    where
        I: std::slice::SliceIndex<[u8]>,
    {
        self.data.get_unchecked(index)
    }
    pub const fn len(&self) -> usize {
        self.data.len()
    }
}
