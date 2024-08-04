use self::{
    authority::{Authority, Repr},
    fragment::Fragment,
    path::Path,
    query::Query,
    scheme::Scheme,
};

pub mod authority;
pub mod fragment;
pub(crate) mod parser;
pub mod path;
pub mod query;
pub mod scheme;

/// See [RFC3986](https://datatracker.ietf.org/doc/html/rfc3986) for more details.
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct Uri {
    pub s: String,
    pub scheme: Option<Scheme>,
    pub authority: Option<Authority>,
    pub path: Option<Path>,
    pub query: Option<Query>,
    pub fragment: Option<Fragment>,
}

impl Uri {
    pub fn parse_exact(uri: impl AsRef<str>) -> Self {
        let s = uri.as_ref();
        let mut parser = parser::Parser::new(s.as_bytes());
        Self {
            s: s.to_owned(),
            scheme: Scheme::parse(&mut parser),
            authority: Authority::parse(&mut parser),
            path: Path::parse(&mut parser),
            query: Query::parse(&mut parser),
            fragment: Fragment::parse(&mut parser),
        }
    }
    pub fn scheme_str(&self) -> Option<&str> {
        self.scheme.as_ref().map(|scheme| {
            let range = scheme.range();
            &self.s[..range.end]
        })
    }
    pub fn authority_str(&self) -> Option<&str> {
        self.authority.as_ref().map(|authority| {
            let repr = authority.repr();
            match repr {
                Repr::Full(range) => unsafe { self.s.get_unchecked(range.start..range.end) },
                Repr::Components {
                    userinfo,
                    host,
                    port,
                } => {
                    let userinfo_str = userinfo
                        .as_ref()
                        .map(|range| unsafe { self.s.get_unchecked(..range.end) });
                    let host_str = host
                        .as_ref()
                        .map(|range| unsafe { self.s.get_unchecked(range.start..range.end) });
                    let port_str = port
                        .as_ref()
                        .map(|range| unsafe { self.s.get_unchecked(range.start..) });
                    todo!()
                }
            }
        })
    }
    pub fn path_str(&self) -> Option<&str> {
        self.path.as_ref().map(|path| {
            let range = path.range();
            match (range.start, range.end) {
                (start, 0) => unsafe { self.s.get_unchecked(start..) },
                (start, end) => unsafe { self.s.get_unchecked(start..end) }
            }
        })
    }
    pub fn query_str(&self) -> Option<&str> {
        self.query.as_ref().map(|query| {
            let range = query.range();
            match (range.start, range.end) {
                (start, 0) => unsafe { self.s.get_unchecked(start..) },
                (start, end) => unsafe { self.s.get_unchecked(start..end)}
            }
        })
    }
    pub fn fragment_str(&self) -> Option<&str> {
        self.fragment.as_ref().map(|fragment| {
            let range = fragment.range();
            unsafe { self.s.get_unchecked(range.start..) }
        })
    }
}

impl std::str::FromStr for Uri {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_exact(s))
    }
}

#[cfg(test)]
mod tests {
    use super::Uri;

    #[test]
    fn parse_rfc3986_uri() {
        const URI: &str = "https://datatracker.ietf.org/doc/html/rfc3986";
        let uri = Uri::parse_exact(URI);
        assert_eq!(uri.scheme_str(), Some("https"));
        assert_eq!(uri.authority_str(), Some("datatracker.ietf.org"));
        assert_eq!(uri.path_str(), Some("/doc/html/rfc3986"));
        assert_eq!(uri.query_str(), None);
        assert_eq!(uri.fragment_str(), None);
    }
}
