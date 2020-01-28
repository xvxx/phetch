#[macro_use]
extern crate criterion;
use criterion::Criterion;

use phetch::{menu, ui::View};
use std::fs;

fn parse(file: &str) -> menu::Menu {
    let raw = fs::read_to_string(file).unwrap();
    menu::parse("benchmark", raw)
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut menu = parse("benches/sdf.txt");
    c.bench_function("render sdf.org", |b| b.iter(|| menu.render()));

    let mut menu = parse("benches/rpod.txt");
    c.bench_function("render RPoD", |b| b.iter(|| menu.render()));

    let mut menu = parse("benches/unix.txt");
    c.bench_function("render UNIX doc", |b| b.iter(|| menu.render()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
