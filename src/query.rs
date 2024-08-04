use super::parser::{Parser, State};
use std::ops::Range;

/// query       = *( pchar / "/" / "?" )
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query(Range<usize>);

impl Query {
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let start = 'start: {
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b'?' {
                    parser.skip(1);
                    break 'start parser.current_position();
                }
                parser.next()
            }
            return None;
        };
        let end = 'end: {
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b'#' && start != 0 {
                    break 'end parser.current_position();
                }
                parser.next()
            }
            0
        };
        Some(Self(start..end))
    }
    pub fn range(&self) -> &Range<usize> {
        &self.0
    }
}
