use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fast_vec::Vector3;
use nalgebra::Vector3 as NalgebraVector3;
use rand::Rng;

fn random_vector3() -> Vector3 {
    let mut rng = rand::thread_rng();
    Vector3::new(rng.gen(), rng.gen(), rng.gen())
}

fn random_nalgebra_vector3() -> NalgebraVector3<f64> {
    let mut rng = rand::thread_rng();
    NalgebraVector3::new(rng.gen(), rng.gen(), rng.gen())
}

fn benchmark_new(c: &mut Criterion) {
    c.bench_function("fast_vec_new", |b| {
        b.iter(|| Vector3::new(1.0, 2.0, 3.0));
    });
    c.bench_function("nalgebra_new", |b| {
        b.iter(|| NalgebraVector3::new(1.0, 2.0, 3.0));
    });
}

fn benchmark_zeros(c: &mut Criterion) {
    c.bench_function("fast_vec_zeros", |b| {
        b.iter(|| Vector3::zeros());
    });
    c.bench_function("nalgebra_zeros", |b| {
        b.iter(|| NalgebraVector3::zeros());
    });
}

fn benchmark_add(c: &mut Criterion) {
    let a = random_vector3();
    let b = random_vector3();
    let na = random_nalgebra_vector3();
    let nb = random_nalgebra_vector3();

    c.bench_function("fast_vec_add", |b| {
        b.iter(|| black_box(a + b));
    });
    c.bench_function("nalgebra_add", |b| {
        b.iter(|| black_box(na + nb));
    });
}

fn benchmark_sub(c: &mut Criterion) {
    let a = random_vector3();
    let b = random_vector3();
    let na = random_nalgebra_vector3();
    let nb = random_nalgebra_vector3();

    c.bench_function("fast_vec_sub", |b| {
        b.iter(|| black_box(a - b));
    });
    c.bench_function("nalgebra_sub", |b| {
        b.iter(|| black_box(na - nb));
    });
}

fn benchmark_mul_scalar(c: &mut Criterion) {
    let a = random_vector3();
    let na = random_nalgebra_vector3();

    c.bench_function("fast_vec_mul_scalar", |b| {
        b.iter(|| black_box(a * 2.5));
    });
    c.bench_function("nalgebra_mul_scalar", |b| {
        b.iter(|| black_box(na * 2.5));
    });
}

fn benchmark_div_scalar(c: &mut Criterion) {
    let a = random_vector3();
    let na = random_nalgebra_vector3();

    c.bench_function("fast_vec_div_scalar", |b| {
        b.iter(|| black_box(a / 2.5));
    });
    c.bench_function("nalgebra_div_scalar", |b| {
        b.iter(|| black_box(na / 2.5));
    });
}

fn benchmark_neg(c: &mut Criterion) {
    let a = random_vector3();
    let na = random_nalgebra_vector3();

    c.bench_function("fast_vec_neg", |b| {
        b.iter(|| black_box(-a));
    });
    c.bench_function("nalgebra_neg", |b| {
        b.iter(|| black_box(-na));
    });
}

fn benchmark_dot(c: &mut Criterion) {
    let a = random_vector3();
    let b = random_vector3();
    let na = random_nalgebra_vector3();
    let nb = random_nalgebra_vector3();

    c.bench_function("fast_vec_dot", |b| {
        b.iter(|| black_box(a.dot(b)));
    });
    c.bench_function("nalgebra_dot", |b| {
        b.iter(|| black_box(na.dot(&nb)));
    });
}

fn benchmark_magnitude_squared(c: &mut Criterion) {
    let a = random_vector3();
    let na = random_nalgebra_vector3();

    c.bench_function("fast_vec_magnitude_squared", |b| {
        b.iter(|| black_box(a.magnitude_squared()));
    });
    c.bench_function("nalgebra_magnitude_squared", |b| {
        b.iter(|| black_box(na.magnitude_squared()));
    });
}

fn benchmark_magnitude(c: &mut Criterion) {
    let a = random_vector3();
    let na = random_nalgebra_vector3();

    c.bench_function("fast_vec_magnitude", |b| {
        b.iter(|| black_box(a.magnitude()));
    });
    c.bench_function("nalgebra_magnitude", |b| {
        b.iter(|| black_box(na.magnitude()));
    });
}

fn benchmark_normalize(c: &mut Criterion) {
    let a = random_vector3();
    let na = random_nalgebra_vector3();

    c.bench_function("fast_vec_normalize", |b| {
        b.iter(|| black_box(a.normalize()));
    });
    c.bench_function("nalgebra_normalize", |b| {
        b.iter(|| black_box(na.normalize()));
    });
}

fn benchmark_cross(c: &mut Criterion) {
    let a = random_vector3();
    let b = random_vector3();
    let na = random_nalgebra_vector3();
    let nb = random_nalgebra_vector3();

    c.bench_function("fast_vec_cross", |b| {
        b.iter(|| black_box(a.cross(b)));
    });
    c.bench_function("nalgebra_cross", |b| {
        b.iter(|| black_box(na.cross(&nb)));
    });
}

fn benchmark_distance(c: &mut Criterion) {
    let a = random_vector3();
    let b = random_vector3();
    let na = random_nalgebra_vector3();
    let nb = random_nalgebra_vector3();

    c.bench_function("fast_vec_distance", |b| {
        b.iter(|| black_box(a.distance(b)));
    });
    c.bench_function("nalgebra_distance", |b| {
        b.iter(|| black_box(na.distance(&nb)));
    });
}

fn benchmark_distance_squared(c: &mut Criterion) {
    let a = random_vector3();
    let b = random_vector3();
    let na = random_nalgebra_vector3();
    let nb = random_nalgebra_vector3();

    c.bench_function("fast_vec_distance_squared", |b| {
        b.iter(|| black_box(a.distance_squared(b)));
    });
    c.bench_function("nalgebra_distance_squared", |b| {
        b.iter(|| black_box(na.distance_squared(&nb)));
    });
}

fn benchmark_getters(c: &mut Criterion) {
    let a = random_vector3();
    let na = random_nalgebra_vector3();

    c.bench_function("fast_vec_getters", |b| {
        b.iter(|| {
            black_box(a.x());
            black_box(a.y());
            black_box(a.z());
        });
    });
    c.bench_function("nalgebra_getters", |b| {
        b.iter(|| {
            black_box(na.x);
            black_box(na.y);
            black_box(na.z);
        });
    });
}

fn benchmark_setters(c: &mut Criterion) {
    let mut a = random_vector3();
    let mut na = random_nalgebra_vector3();
    let mut rng = rand::thread_rng();

    c.bench_function("fast_vec_setters", |b| {
        b.iter(|| {
            black_box(a.set_x(rng.gen()));
            black_box(a.set_y(rng.gen()));
            black_box(a.set_z(rng.gen()));
        });
    });
    c.bench_function("nalgebra_setters", |b| {
        b.iter(|| {
            black_box(na.x = rng.gen());
            black_box(na.y = rng.gen());
            black_box(na.z = rng.gen());
        });
    });
}

criterion_group!(
    benches,
    benchmark_new,
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
