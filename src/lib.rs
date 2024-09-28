#![forbid(unsafe_code)]

#[doc = include_str!("../README.md")]
pub mod error;
mod parsing;

use self::{
    error::Error,
    parsing::{lexer::token::span::Span, UriParser},
};

/// See [Authority](https://datatracker.ietf.org/doc/html/rfc3986#section-3.2) for more details.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Authority {
    input: String,
    userinfo: Option<Span>,
    host: Span,
    port: Option<Span>,
}

impl Authority {
    pub fn userinfo(&self) -> Option<&str> {
        self.userinfo.map(|span| {
            let start = span.start();
            let length = span.length();
            &self.input[start..start + length]
        })
    }
    pub fn host(&self) -> &str {
        let span = self.host;
        let start = span.start();
        let length = span.length();
        &self.input[start..start + length]
    }
    pub fn port_str(&self) -> Option<&str> {
        self.port.map(|span| {
            let start = span.start();
            let length = span.length();
            &self.input[start..start + length]
        })
    }
    pub fn port(&self) -> Option<u16> {
        self.port_str()
            .map(|port| port.parse::<u16>().unwrap_or_default())
    }
}

/// See [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986) for more details.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri {
    input: String,
    scheme: Option<Span>,
    authority: Option<Authority>,
    path: Span,
    query: Option<Span>,
    fragment: Option<Span>,
}

impl Uri {
    pub fn scheme(&self) -> Option<&str> {
        self.scheme.map(|span| {
            let start = span.start();
            let length = span.length();
            &self.input[start..start + length]
        })
    }
    pub fn authority(&self) -> Option<&Authority> {
        self.authority.as_ref()
    }

    pub fn path(&self) -> &str {
        let span = self.path;
        let start = span.start();
        let length = span.length();
        &self.input[start..start + length]
    }

    pub fn query(&self) -> Option<&str> {
        self.query.map(|span| {
            let start = span.start();
            let length = span.length();
            &self.input[start..start + length]
        })
    }

    pub fn fragment(&self) -> Option<&str> {
        self.fragment.map(|span| {
            let start = span.start();
            let length = span.length();
            &self.input[start..start + length]
        })
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
        assert_eq!(
            uri.authority().map(|authority| authority.host()),
            Some("datatracker.ietf.org")
        );
        assert_eq!(uri.path(), "/doc/html/rfc3986");
    }
    #[test]
    fn make_scheme_and_authority_lowercase() {
        const URI: &str = "HTTPS://DATATRACKER.IETF.ORG/DOC/html/rfc3986";
        let uri = URI.parse::<super::Uri>().unwrap();
        assert_eq!(uri.scheme(), Some("https"));
        assert_eq!(
            uri.authority().map(|authority| authority.host()),
            Some("datatracker.ietf.org")
        );
        assert_ne!(uri.path(), "/doc/html/rfc3986");
    }
}
