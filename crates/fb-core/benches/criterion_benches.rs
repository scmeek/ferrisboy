use criterion::{Criterion, criterion_group, criterion_main};
use fb_core::prelude::{Config, Emulator};
use std::hint::black_box;

fn bench_emulator(c: &mut Criterion) {
    c.bench_function("emulator_new", |b| {
        b.iter(|| {
            let config = Config {};
            let result = Emulator::new(black_box(&config));
            black_box(result)
        });
    });
}

criterion_group!(benches, bench_emulator);
criterion_main!(benches);
