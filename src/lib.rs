#![forbid(unsafe_code)]

#[doc = include_str!("../README.md")]
pub mod error;
mod parsing;

use self::{
    error::Error,
    parsing::{lexer::token::span::Span, UriParser},
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Authority {
    pub userinfo: Option<Span>,
    pub host: Span,
    pub port: Option<Span>,
}

/// See [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986) for more details.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri {
    source: String,
    scheme: Option<Span>,
    authority: Option<Authority>,
    path: Span,
    query: Option<Span>,
    fragment: Option<Span>,
}

impl Uri {
    pub fn scheme(&self) -> Option<&str> {
        self.get_optional_slice(self.scheme)
    }
    pub fn userinfo(&self) -> Option<&str> {
        self.map_authority(|authority| self.get_optional_slice(authority.userinfo))
    }
    pub fn host(&self) -> Option<&str> {
        self.map_authority(|authority| self.get_optional_slice(Some(authority.host)))
    }
    pub fn port_str(&self) -> Option<&str> {
        self.map_authority(|authority| self.get_optional_slice(authority.port))
    }
    pub fn port(&self) -> Option<u16> {
        self.port_str()
            .map(|port| port.parse::<u16>().unwrap_or_default())
    }

    pub fn path(&self) -> &str {
        self.get_slice(self.path)
    }

    pub fn query(&self) -> Option<&str> {
        self.get_optional_slice(self.query)
    }

    pub fn fragment(&self) -> Option<&str> {
        self.get_optional_slice(self.fragment)
    }
    fn map_authority<F, U>(&self, f: F) -> Option<U>
    where
        F: FnOnce(&Authority) -> Option<U>,
    {
        self.authority.as_ref().and_then(f)
    }
    fn get_optional_slice(&self, span: Option<Span>) -> Option<&str> {
        span.map(|span| self.get_slice(span))
    }
    fn get_slice(&self, span: Span) -> &str {
        &self.source[span.start()..span.end()]
    }
}

impl std::str::FromStr for Uri {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parser = UriParser::new(s)?;
        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_rfc3986_uri() {
        const URI: &str = "https://datatracker.ietf.org/doc/html/rfc3986";
        let uri = URI.parse::<super::Uri>().unwrap();
        assert_eq!(uri.scheme(), Some("https"));
        assert_eq!(uri.host(), Some("datatracker.ietf.org"));
        assert_eq!(uri.path(), "/doc/html/rfc3986");
    }
    #[test]
    fn make_scheme_and_authority_lowercase() {
        const URI: &str = "HTTPS://DATATRACKER.IETF.ORG/DOC/html/rfc3986";
        let uri = URI.parse::<super::Uri>().unwrap();
        assert_eq!(uri.scheme(), Some("https"));
        assert_eq!(uri.host(), Some("datatracker.ietf.org"));
        assert_ne!(uri.path(), "/doc/html/rfc3986");
    }
}
