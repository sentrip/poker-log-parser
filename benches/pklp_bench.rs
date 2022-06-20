#[path = "../src/pklp.rs"]
mod pklp;
use pklp::{parse_string, str_to_json};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let s = std::fs::read_to_string("data/example/pokerstars_example.txt").unwrap();
    let sr = s.as_str();
    c.bench_function("pokerstars - parse_string", |b| b.iter(|| parse_string(black_box(sr))));
    c.bench_function("pokerstars - str_to_json", |b| b.iter(|| str_to_json(black_box(sr))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
