use criterion::{black_box, criterion_group, criterion_main, Criterion};
use phantom_http::Uri;

pub fn parse_rfc3986_uri(criterion: &mut Criterion) {
    criterion.bench_function("parse_rfc3986_uri", |bencher| bencher.iter(|| {
        black_box(for _ in 0..100 {
            "https://datatracker.ietf.org/doc/html/rfc3986".parse::<Uri>().unwrap();
        })
    }));
}

criterion_group!(benches, parse_rfc3986_uri);
criterion_main!(benches);