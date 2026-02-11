use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_vec::Vector2;
use nalgebra::Vector2 as NalgebraVector2;

fn benchmark_new(c: &mut Criterion) {
    c.bench_function("fast_vec2_new", |b| {
        b.iter(|| Vector2::new(1.0, 2.0));
    });
    c.bench_function("nalgebra2_new", |b| {
        b.iter(|| NalgebraVector2::new(1.0, 2.0));
    });
}

fn benchmark_zeros(c: &mut Criterion) {
    c.bench_function("fast_vec2_zeros", |b| {
        b.iter(|| Vector2::zeros());
    });
    c.bench_function("nalgebra2_zeros", |b| {
        b.iter(|| NalgebraVector2::<f64>::zeros());
    });
}

fn benchmark_add(c: &mut Criterion) {
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(3.0, 4.0);
    let nv1 = NalgebraVector2::new(1.0, 2.0);
    let nv2 = NalgebraVector2::new(3.0, 4.0);

    c.bench_function("fast_vec2_add", |b| {
        b.iter(|| black_box(black_box(v1) + black_box(v2)));
    });
    c.bench_function("nalgebra2_add", |b| {
        b.iter(|| black_box(black_box(nv1) + black_box(nv2)));
    });
}

fn benchmark_sub(c: &mut Criterion) {
    let v1 = Vector2::new(4.0, 6.0);
    let v2 = Vector2::new(1.0, 2.0);
    let nv1 = NalgebraVector2::new(4.0, 6.0);
    let nv2 = NalgebraVector2::new(1.0, 2.0);

    c.bench_function("fast_vec2_sub", |b| {
        b.iter(|| black_box(black_box(v1) - black_box(v2)));
    });
    c.bench_function("nalgebra2_sub", |b| {
        b.iter(|| black_box(black_box(nv1) - black_box(nv2)));
    });
}

fn benchmark_mul_scalar(c: &mut Criterion) {
    let v = Vector2::new(1.0, 2.0);
    let nv = NalgebraVector2::new(1.0, 2.0);

    c.bench_function("fast_vec2_mul_scalar", |b| {
        b.iter(|| black_box(black_box(v) * 2.5));
    });
    c.bench_function("nalgebra2_mul_scalar", |b| {
        b.iter(|| black_box(black_box(nv) * 2.5));
    });
}

fn benchmark_div_scalar(c: &mut Criterion) {
    let v = Vector2::new(2.5, 5.0);
    let nv = NalgebraVector2::new(2.5, 5.0);

    c.bench_function("fast_vec2_div_scalar", |b| {
        b.iter(|| black_box(black_box(v) / 2.5));
    });
    c.bench_function("nalgebra2_div_scalar", |b| {
        b.iter(|| black_box(black_box(nv) / 2.5));
    });
}

fn benchmark_neg(c: &mut Criterion) {
    let v = Vector2::new(1.0, -2.0);
    let nv = NalgebraVector2::new(1.0, -2.0);

    c.bench_function("fast_vec2_neg", |b| {
        b.iter(|| black_box(-black_box(v)));
    });
    c.bench_function("nalgebra2_neg", |b| {
        b.iter(|| black_box(-black_box(nv)));
    });
}

fn benchmark_dot(c: &mut Criterion) {
    let v1 = Vector2::new(1.0, 2.0);
    let v2 = Vector2::new(3.0, 4.0);
    let nv1 = NalgebraVector2::new(1.0, 2.0);
    let nv2 = NalgebraVector2::new(3.0, 4.0);

    c.bench_function("fast_vec2_dot", |b| {
        b.iter(|| black_box(black_box(v1).dot(black_box(v2))));
    });
    c.bench_function("nalgebra2_dot", |b| {
        b.iter(|| black_box(black_box(nv1).dot(&black_box(nv2))));
    });
}

fn benchmark_magnitude_squared(c: &mut Criterion) {
    let v = Vector2::new(3.0, 4.0);
    let nv = NalgebraVector2::new(3.0, 4.0);

    c.bench_function("fast_vec2_magnitude_squared", |b| {
        b.iter(|| black_box(black_box(v).magnitude_squared()));
    });
    c.bench_function("nalgebra2_magnitude_squared", |b| {
        b.iter(|| black_box(black_box(nv).magnitude_squared()));
    });
}

fn benchmark_magnitude(c: &mut Criterion) {
    let v = Vector2::new(3.0, 4.0);
    let nv = NalgebraVector2::new(3.0, 4.0);

    c.bench_function("fast_vec2_magnitude", |b| {
        b.iter(|| black_box(black_box(v).magnitude()));
    });
    c.bench_function("nalgebra2_magnitude", |b| {
        b.iter(|| black_box(black_box(nv).magnitude()));
    });
}

fn benchmark_normalize(c: &mut Criterion) {
    let v = Vector2::new(3.0, 4.0);
    let nv = NalgebraVector2::new(3.0, 4.0);

    c.bench_function("fast_vec2_normalize", |b| {
        b.iter(|| black_box(black_box(v).normalize()));
    });
    c.bench_function("nalgebra2_normalize", |b| {
        b.iter(|| black_box(black_box(nv).normalize()));
    });
}

fn benchmark_cross(c: &mut Criterion) {
    let v1 = Vector2::new(1.0, 0.0);
    let v2 = Vector2::new(0.0, 1.0);
    let nv1 = NalgebraVector2::new(1.0, 0.0);
    let nv2 = NalgebraVector2::new(0.0, 1.0);

    c.bench_function("fast_vec2_cross", |b| {
        b.iter(|| black_box(black_box(v1).cross(black_box(v2))));
    });
    c.bench_function("nalgebra2_cross", |b| {
        b.iter(|| black_box(black_box(nv1).perp(&black_box(nv2))));
    });
}

fn benchmark_distance(c: &mut Criterion) {
    let v1 = Vector2::new(0.0, 0.0);
    let v2 = Vector2::new(3.0, 4.0);
    let nv1 = NalgebraVector2::new(0.0, 0.0);
    let nv2 = NalgebraVector2::new(3.0, 4.0);

    c.bench_function("fast_vec2_distance", |b| {
        b.iter(|| black_box(black_box(v1).distance(black_box(v2))));
    });
    c.bench_function("nalgebra2_distance", |b| {
        b.iter(|| black_box(black_box(nv1).metric_distance(&black_box(nv2))));
    });
}

fn benchmark_distance_squared(c: &mut Criterion) {
    let v1 = Vector2::new(0.0, 0.0);
    let v2 = Vector2::new(3.0, 4.0);
    let nv1 = NalgebraVector2::new(0.0, 0.0);
    let nv2 = NalgebraVector2::new(3.0, 4.0);

    c.bench_function("fast_vec2_distance_squared", |b| {
        b.iter(|| black_box(black_box(v1).distance_squared(black_box(v2))));
    });
    c.bench_function("nalgebra2_distance_squared", |b| {
        b.iter(|| black_box((black_box(nv1) - black_box(nv2)).magnitude_squared()));
    });
}

fn benchmark_getters(c: &mut Criterion) {
    let v = Vector2::new(1.0, 2.0);
    let nv = NalgebraVector2::new(1.0, 2.0);

    c.bench_function("fast_vec2_getters", |b| {
        b.iter(|| {
            black_box(black_box(v).x());
            black_box(black_box(v).y());
        });
    });
    c.bench_function("nalgebra2_getters", |b| {
        b.iter(|| {
            black_box(black_box(nv).x);
            black_box(black_box(nv).y);
        });
    });
}

fn benchmark_setters(c: &mut Criterion) {
    let v = Vector2::new(1.0, 2.0);
    let nv = NalgebraVector2::new(1.0, 2.0);

    c.bench_function("fast_vec2_setters", |b| {
        b.iter(|| {
            black_box(black_box(v).set_x(4.0));
            black_box(black_box(v).set_y(5.0));
        });
    });
    c.bench_function("nalgebra2_setters", |b| {
        b.iter(|| {
            black_box(black_box(nv).x = 4.0);
            black_box(black_box(nv).y = 5.0);
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10_000);
    targets =  benchmark_new,
    benchmark_zeros,
    benchmark_add,
    benchmark_sub,
    benchmark_mul_scalar,
    benchmark_div_scalar,
    benchmark_neg,
    benchmark_dot,
    benchmark_magnitude_squared,
    benchmark_magnitude,
    benchmark_normalize,
    benchmark_cross,
    benchmark_distance,
    benchmark_distance_squared,
    benchmark_getters,
    benchmark_setters
);
criterion_main!(benches);
