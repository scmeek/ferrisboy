use criterion::{Criterion, criterion_group, criterion_main};
// use fb_core::add;
// use std::hint::black_box;

fn empty(_c: &mut Criterion) {}
// fn bench_add(c: &mut Criterion) {
//     c.bench_function("my_function", |b| {
//         b.iter(|| add(black_box(42), black_box(58)));
//     });
// }

criterion_group!(benches, empty);
criterion_main!(benches);
