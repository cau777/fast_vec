use criterion::{black_box, criterion_group, criterion_main, Criterion};
use fast_vec::Vector3;
use nalgebra::Vector3 as NalgebraVector3;

fn benchmark_new(c: &mut Criterion) {
    c.bench_function("fast_vec3_new", |b| {
        b.iter(|| Vector3::new(1.0, 2.0, 3.0));
    });
    c.bench_function("nalgebra3_new", |b| {
        b.iter(|| NalgebraVector3::new(1.0, 2.0, 3.0));
    });
}

fn benchmark_zeros(c: &mut Criterion) {
    c.bench_function("fast_vec3_zeros", |b| {
        b.iter(|| Vector3::zeros());
    });
    c.bench_function("nalgebra3_zeros", |b| {
        b.iter(|| NalgebraVector3::<f64>::zeros());
    });
}

fn benchmark_add(c: &mut Criterion) {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);
    let nv1 = NalgebraVector3::new(1.0, 2.0, 3.0);
    let nv2 = NalgebraVector3::new(4.0, 5.0, 6.0);

    c.bench_function("fast_vec3_add", |b| {
        b.iter(|| black_box(black_box(v1) + black_box(v2)));
    });
    c.bench_function("nalgebra3_add", |b| {
        b.iter(|| black_box(black_box(nv1) + black_box(nv2)));
    });
}

fn benchmark_sub(c: &mut Criterion) {
    let v1 = Vector3::new(4.0, 5.0, 6.0);
    let v2 = Vector3::new(1.0, 2.0, 3.0);
    let nv1 = NalgebraVector3::new(4.0, 5.0, 6.0);
    let nv2 = NalgebraVector3::new(1.0, 2.0, 3.0);

    c.bench_function("fast_vec3_sub", |b| {
        b.iter(|| black_box(black_box(v1) - black_box(v2)));
    });
    c.bench_function("nalgebra3_sub", |b| {
        b.iter(|| black_box(black_box(nv1) - black_box(nv2)));
    });
}

fn benchmark_mul_scalar(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let nv = NalgebraVector3::new(1.0, 2.0, 3.0);

    c.bench_function("fast_vec3_mul_scalar", |b| {
        b.iter(|| black_box(black_box(v) * 2.5));
    });
    c.bench_function("nalgebra3_mul_scalar", |b| {
        b.iter(|| black_box(black_box(nv) * 2.5));
    });
}

fn benchmark_div_scalar(c: &mut Criterion) {
    let v = Vector3::new(2.5, 5.0, 7.5);
    let nv = NalgebraVector3::new(2.5, 5.0, 7.5);

    c.bench_function("fast_vec3_div_scalar", |b| {
        b.iter(|| black_box(black_box(v) / 2.5));
    });
    c.bench_function("nalgebra3_div_scalar", |b| {
        b.iter(|| black_box(black_box(nv) / 2.5));
    });
}

fn benchmark_neg(c: &mut Criterion) {
    let v = Vector3::new(1.0, -2.0, 3.0);
    let nv = NalgebraVector3::new(1.0, -2.0, 3.0);

    c.bench_function("fast_vec3_neg", |b| {
        b.iter(|| black_box(-black_box(v)));
    });
    c.bench_function("nalgebra3_neg", |b| {
        b.iter(|| black_box(-black_box(nv)));
    });
}

fn benchmark_dot(c: &mut Criterion) {
    let v1 = Vector3::new(1.0, 2.0, 3.0);
    let v2 = Vector3::new(4.0, 5.0, 6.0);
    let nv1 = NalgebraVector3::new(1.0, 2.0, 3.0);
    let nv2 = NalgebraVector3::new(4.0, 5.0, 6.0);

    c.bench_function("fast_vec3_dot", |b| {
        b.iter(|| black_box(black_box(v1).dot(black_box(v2))));
    });
    c.bench_function("nalgebra3_dot", |b| {
        b.iter(|| black_box(black_box(nv1).dot(&black_box(nv2))));
    });
}

fn benchmark_magnitude_squared(c: &mut Criterion) {
    let v = Vector3::new(3.0, 4.0, 5.0);
    let nv = NalgebraVector3::new(3.0, 4.0, 5.0);

    c.bench_function("fast_vec3_magnitude_squared", |b| {
        b.iter(|| black_box(black_box(v).magnitude_squared()));
    });
    c.bench_function("nalgebra3_magnitude_squared", |b| {
        b.iter(|| black_box(black_box(nv).magnitude_squared()));
    });
}

fn benchmark_magnitude(c: &mut Criterion) {
    let v = Vector3::new(3.0, 4.0, 5.0);
    let nv = NalgebraVector3::new(3.0, 4.0, 5.0);

    c.bench_function("fast_vec3_magnitude", |b| {
        b.iter(|| black_box(black_box(v).magnitude()));
    });
    c.bench_function("nalgebra3_magnitude", |b| {
        b.iter(|| black_box(black_box(nv).magnitude()));
    });
}

fn benchmark_normalize(c: &mut Criterion) {
    let v = Vector3::new(3.0, 4.0, 5.0);
    let nv = NalgebraVector3::new(3.0, 4.0, 5.0);

    c.bench_function("fast_vec3_normalize", |b| {
        b.iter(|| black_box(black_box(v).normalize()));
    });
    c.bench_function("nalgebra3_normalize", |b| {
        b.iter(|| black_box(black_box(nv).normalize()));
    });
}

fn benchmark_cross(c: &mut Criterion) {
    let v1 = Vector3::new(1.0, 0.0, 0.0);
    let v2 = Vector3::new(0.0, 1.0, 0.0);
    let nv1 = NalgebraVector3::new(1.0, 0.0, 0.0);
    let nv2 = NalgebraVector3::new(0.0, 1.0, 0.0);

    c.bench_function("fast_vec3_cross", |b| {
        b.iter(|| black_box(black_box(v1).cross(black_box(v2))));
    });
    c.bench_function("nalgebra3_cross", |b| {
        b.iter(|| black_box(black_box(nv1).cross(&black_box(nv2))));
    });
}

fn benchmark_distance(c: &mut Criterion) {
    let v1 = Vector3::new(0.0, 0.0, 0.0);
    let v2 = Vector3::new(3.0, 4.0, 5.0);
    let nv1 = NalgebraVector3::new(0.0, 0.0, 0.0);
    let nv2 = NalgebraVector3::new(3.0, 4.0, 5.0);

    c.bench_function("fast_vec3_distance", |b| {
        b.iter(|| black_box(black_box(v1).distance(black_box(v2))));
    });
    c.bench_function("nalgebra3_distance", |b| {
        b.iter(|| black_box(black_box(nv1).metric_distance(&black_box(nv2))));
    });
}

fn benchmark_distance_squared(c: &mut Criterion) {
    let v1 = Vector3::new(0.0, 0.0, 0.0);
    let v2 = Vector3::new(3.0, 4.0, 5.0);
    let nv1 = NalgebraVector3::new(0.0, 0.0, 0.0);
    let nv2 = NalgebraVector3::new(3.0, 4.0, 5.0);

    c.bench_function("fast_vec3_distance_squared", |b| {
        b.iter(|| black_box(black_box(v1).distance_squared(black_box(v2))));
    });
    c.bench_function("nalgebra3_distance_squared", |b| {
        b.iter(|| black_box((black_box(nv1) - black_box(nv2)).magnitude_squared()));
    });
}

fn benchmark_getters(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let nv = NalgebraVector3::new(1.0, 2.0, 3.0);

    c.bench_function("fast_vec3_getters", |b| {
        b.iter(|| {
            black_box(black_box(v).x());
            black_box(black_box(v).y());
            black_box(black_box(v).z());
        });
    });
    c.bench_function("nalgebra3_getters", |b| {
        b.iter(|| {
            black_box(black_box(nv).x);
            black_box(black_box(nv).y);
            black_box(black_box(nv).z);
        });
    });
}

fn benchmark_setters(c: &mut Criterion) {
    let v = Vector3::new(1.0, 2.0, 3.0);
    let nv = NalgebraVector3::new(1.0, 2.0, 3.0);

    c.bench_function("fast_vec3_setters", |b| {
        b.iter(|| {
            black_box(black_box(v).set_x(4.0));
            black_box(black_box(v).set_y(5.0));
            black_box(black_box(v).set_z(6.0));
        });
    });
    c.bench_function("nalgebra3_setters", |b| {
        b.iter(|| {
            black_box(black_box(nv).x = 4.0);
            black_box(black_box(nv).y = 5.0);
            black_box(black_box(nv).z = 6.0);
        });
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(10_000);
    targets = benchmark_new,
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
