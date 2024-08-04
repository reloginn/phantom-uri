use super::parser::{Parser, State};
use std::ops::RangeTo;

/// scheme = ALPHA *( ALPHA / DIGIT / «+» / «-» / «.» )
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Scheme(RangeTo<usize>);

impl Scheme {
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b':' {
                let end = parser.current_position();
                return Some(Self(..end));
            }
            parser.next()
        }
        None
    }
    pub fn range(&self) -> &RangeTo<usize> {
        &self.0
    }
}
