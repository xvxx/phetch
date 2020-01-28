#[macro_use]
extern crate criterion;
use criterion::Criterion;

use phetch::{config::Config, text, ui::View};
use std::fs;

fn parse(cfg: &Config, txt: String) -> text::Text {
    text::Text::from("benchmark", txt, cfg, false)
}

fn criterion_benchmark(c: &mut Criterion) {
    let cfg = Config::default();

    if let Ok(raw) = fs::read_to_string("benches/rfc1436.txt") {
        let mut txt = parse(&cfg, raw);
        c.bench_function("render rfc text", |b| b.iter(|| txt.render()));
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
