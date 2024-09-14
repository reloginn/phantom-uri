# What is it?
This is an implementation of the URI parser (RFC3986)
# When is the stable version?
At the moment the implementation has rather poor performance due to the large number of allocations and reallocations, and also the `normalize_path` function needed to normalize paths is not done in the implementation. My goals for the next commit are to improve performance to the maximum, I don't guarantee that I won't use `unsafe` but I will try to avoid it.
# Example
```rust
const URI: &str = "https://datatracker.ietf.org/doc/html/rfc3986";
let uri = URI.parse::<phantom_uri::Uri>().unwrap();
assert_eq!(uri.scheme(), Some("https"));
assert_eq!(uri.authority().unwrap().host(), "datatracker.ietf.org");
assert_eq!(uri.path(), "/doc/html/rfc3986");
```