pub mod error;
mod parsing;

use self::{error::ParseUriError, parsing::{UriParser, lexer::Lexer}};

/// See [Authority](https://datatracker.ietf.org/doc/html/rfc3986#section-3.2) for more details.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Authority {
    userinfo: Option<String>,
    host: String,
    port: Option<u16>,
}

impl Authority {
    pub fn userinfo(&self) -> Option<&str> {
        self.userinfo.as_deref()
    }
    pub fn host(&self) -> &str {
        self.host.as_ref()
    }
    pub fn port(&self) -> Option<u16> {
        self.port
    }
}

/// See [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986) for more details.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri {
    scheme: Option<String>,
    authority: Option<Authority>,
    path: String,
    query: Option<String>,
    fragment: Option<String>,
}

impl Uri {
    pub fn scheme(&self) -> Option<&str> {
        self.scheme.as_deref()
    }
    pub fn authority(&self) -> Option<&Authority> {
        self.authority.as_ref()
    }

    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    pub fn fragment(&self) -> Option<&str> {
        self.fragment.as_deref()
    }
}

impl std::str::FromStr for Uri {
    type Err = ParseUriError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = Lexer::new(s);
        let mut tokens = Vec::new();
        while let Ok(Some(token)) = lexer.next_token() {
            tokens.push(token)
        }
        UriParser::new(tokens).parse()
    }
}
