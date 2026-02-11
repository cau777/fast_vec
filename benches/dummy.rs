use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_vec::Vector3;

fn benchmark_dummy(c: &mut Criterion) {
    c.bench_function("dummy_add", |b| {
        let v1 = black_box(Vector3::new(1.0, 2.0, 3.0));
        let v2 = black_box(Vector3::new(4.0, 5.0, 6.0));
        b.iter(|| v1 + v2);
    });
}

criterion_group!(benches, benchmark_dummy);
criterion_main!(benches);
