use criterion::{Criterion, criterion_group, criterion_main};
use fb_core::prelude::{Config, Emulator};
use std::hint::black_box;

fn bench_emulator_init(c: &mut Criterion) {
    c.bench_function("emulator_new", |b| {
        b.iter(|| {
            let config = Config {};
            Emulator::new(black_box(&config))
        });
    });
}

criterion_group!(benches, bench_emulator_init);
criterion_main!(benches);
