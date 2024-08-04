use super::parser::{Parser, State};
use std::ops::{Range, RangeFrom, RangeTo};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Repr {
    Full(Range<usize>),
    Components {
        userinfo: Option<RangeTo<usize>>,
        host: Option<Range<usize>>,
        port: Option<RangeFrom<usize>>,
    },
}

/// authority = [ userinfo «@» ] host [ «:» port ]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority(Repr);

impl Authority {
    pub(super) fn parse(parser: &mut Parser) -> Option<Self> {
        let start = 'start: {
            let mut colon = false;
            let mut slash_and_colon = false;
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b':' {
                    colon = true;
                }
                if byte == b'/' && colon {
                    slash_and_colon = true;
                }
                if byte == b'/' && slash_and_colon {
                    parser.skip(2);
                    break 'start parser.current_position();
                }
                parser.next()
            }
            return None;
        };
        let end = 'end: {
            while parser.state() != State::Eof {
                let byte = parser.get_byte();
                if byte == b'/' || byte == b'?' || byte == b'#' && start != 0 {
                    break 'end parser.current_position();
                }
                parser.next()
            }
            0
        };
        Some(Self(Repr::Full(start..end)))
    }
    pub(super) fn into_components(self, ctx: &str) -> Self {
        match self.repr() {
            Repr::Full(full) => {
                let s = &ctx[full.start..full.end];
                let mut parser = Parser::new(s.as_bytes());
                let userinfo = 'userinfo: {
                    while parser.state() != State::Eof {
                        let byte = parser.get_byte();
                        if byte == b'@' {
                            let end = parser.current_position();
                            break 'userinfo Some(..end);
                        }
                        parser.next()
                    }
                    None
                };
                let host = 'host: {
                    let start = {
                        if parser.get_byte() == b'@' {
                            parser.skip(1)
                        }
                        parser.current_position()
                    };
                    while parser.state() != State::Eof {
                        let byte = parser.get_byte();
                        if byte == b':' {
                            let end = parser.current_position();
                            break 'host Some(start..end);
                        }
                        parser.next()
                    }
                    Some(start..0)
                };
                let port = {
                    if parser.get_byte() == b':' {
                        let start = {
                            parser.skip(1);
                            parser.current_position()
                        };
                        Some(start..)
                    } else {
                        None
                    }
                };
                Self(Repr::Components {
                    userinfo,
                    host,
                    port,
                })
            }
            _ => self,
        }
    }
    pub(super) const fn repr(&self) -> &Repr {
        &self.0
    }
}
