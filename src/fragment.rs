use super::parser::{Parser, State};
use std::ops::RangeFrom;

/// fragment    = *( pchar / "/" / "?" )
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fragment(RangeFrom<usize>);

impl Fragment {
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        // FIXME: There should be no components after `Fragment`
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b'#' {
                let start = {
                    parser.skip(1);
                    parser.current_position()
                };
                return Some(Self(start..));
            }
            parser.next()
        }
        None
    }
    pub(super) fn range(&self) -> &RangeFrom<usize> {
        &self.0
    }
}
