# What is it?
This is a **fast, zero-dependency, #[forbid(unsafe_code)]** implementation of RFC3986 (URI)
# When is the stable version?
Currently, the implementation is not fully compliant with RFC3986 and has no `normalize_path` implementation for the path.
# Example
```rust
const URI: &str = "https://datatracker.ietf.org/doc/html/rfc3986";
let uri = URI.parse::<phantom_uri::Uri>().unwrap();
assert_eq!(uri.scheme(), Some("https"));
assert_eq!(uri.authority().map(|authority| authority.host()), Some("datatracker.ietf.org"));
assert_eq!(uri.path(), "/doc/html/rfc3986");
```
