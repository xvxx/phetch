#[macro_use]
extern crate criterion;
use criterion::Criterion;

use phetch::menu;
use std::fs;

fn parse(txt: &str) -> menu::Menu {
    menu::parse("benchmark", txt.to_string())
}

fn criterion_benchmark(c: &mut Criterion) {
    if let Ok(raw) = fs::read_to_string("benches/sdf.txt") {
        c.bench_function("parse sdf.org", |b| b.iter(|| parse(&raw)));
    }

    if let Ok(raw) = fs::read_to_string("benches/rpod.txt") {
        c.bench_function("parse RPoD", |b| b.iter(|| parse(&raw)));
    }

    if let Ok(raw) = fs::read_to_string("benches/unix.txt") {
        c.bench_function("parse UNIX docs", |b| b.iter(|| parse(&raw)));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
