use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn sr(i: usize) {
    let _ = i.isqrt();
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| sr(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
