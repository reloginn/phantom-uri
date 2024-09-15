> [!WARNING]  
> The implementation may have bugs and other flaws, you can open `Issue` if you find any bug.
# What is it?
This is a **fast, zero-dependency, #[forbid(unsafe_code)]** implementation of RFC3986 (URI)
# Future plans
Make a **zero-copy** parser and implement an optional path normalization feature.
# Example
```rust
const URI: &str = "https://datatracker.ietf.org/doc/html/rfc3986";
let uri = URI.parse::<phantom_uri::Uri>().unwrap();
assert_eq!(uri.scheme(), Some("https"));
assert_eq!(uri.authority().map(|authority| authority.host()), Some("datatracker.ietf.org"));
assert_eq!(uri.path(), "/doc/html/rfc3986");
```
