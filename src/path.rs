use super::parser::{Parser, State};
use std::ops::Range;

/// path          = path-abempty    ; begins with "/" or is empty
///               / path-absolute   ; begins with "/" but not "//"
///               / path-noscheme   ; begins with a non-colon segment
///               / path-rootless   ; begins with a segment
///               / path-empty      ; zero characters
///
/// path-abempty  = *( "/" segment )
/// path-absolute = "/" [ segment-nz *( "/" segment ) ]
/// path-noscheme = segment-nz-nc *( "/" segment )
/// path-rootless = segment-nz *( "/" segment )
/// path-empty    = 0<pchar>
/// segment       = *pchar
/// segment-nz    = 1*pchar
/// segment-nz-nc = 1*( unreserved / pct-encoded / sub-delims / "@" )
///               ; non-zero-length segment without any colon ":"
///
/// pchar         = unreserved / pct-encoded / sub-delims / ":" / "@"
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(Range<usize>);

impl Path {
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let start = {
            let position = parser.current_position();
            if position >= parser.eof() {
                None
            } else {
                Some(position)
            }
        }?;
        while parser.state() != State::Eof {
            let byte = parser.get_byte();
            if byte == b'?' || byte == b'#' {
                let end = parser.current_position();
                return Some(Self(start..end));
            }
            if byte == b':' {
                return Some(Self(start..0));
            }
            parser.next()
        }
        Some(Self(start..0))
    }
    pub(super) fn range(&self) -> &Range<usize> {
        &self.0
    }
}
