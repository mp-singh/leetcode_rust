use criterion::{black_box, criterion_group, criterion_main, Criterion};

use plus_one::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("plus 1", |b| {
        b.iter(|| {
            plus_one(black_box(vec![
                7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9,
                4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 6,
            ]))
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
