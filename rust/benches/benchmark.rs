use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sceua::duan_test_func::*;
use sceua::{minimize, Config};
use std::time::Duration;

#[cfg(feature = "parallel")]
use sceua::minimize_parallel;

fn goldstein_price_config() -> Config {
    Config {
        max_evaluations: 5_000,
        kstop: 5,
        pcento: 0.01,
        seed: 1969,
        complexes: 5,
        ..Config::default()
    }
}

fn rosenbrock_config() -> Config {
    Config {
        max_evaluations: 10_000,
        kstop: 5,
        pcento: 0.0,
        seed: 1969,
        complexes: 5,
        ..Config::default()
    }
}

fn higher_d_config() -> Config {
    Config {
        max_evaluations: 10_000,
        kstop: 5,
        pcento: 0.01,
        seed: 1969,
        complexes: 5,
        ..Config::default()
    }
}

fn bench_goldstein_price(c: &mut Criterion) {
    let config = goldstein_price_config();
    let mut group = c.benchmark_group("goldstein_price");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("serial", |b| {
        b.iter(|| {
            minimize(
                black_box(goldstein_price),
                black_box(&[-2.0, -2.0]),
                black_box(&[2.0, 2.0]),
                config.clone(),
            )
            .unwrap()
        })
    });

    #[cfg(feature = "parallel")]
    group.bench_function("parallel", |b| {
        b.iter(|| {
            minimize_parallel(
                black_box(goldstein_price),
                black_box(&[-2.0, -2.0]),
                black_box(&[2.0, 2.0]),
                config.clone(),
            )
            .unwrap()
        })
    });

    group.finish();
}

fn bench_rosenbrock(c: &mut Criterion) {
    let config = rosenbrock_config();
    let mut group = c.benchmark_group("rosenbrock");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("serial", |b| {
        b.iter(|| {
            minimize(
                black_box(rosenbrock),
                black_box(&[-5.0, -5.0]),
                black_box(&[5.0, 5.0]),
                config.clone(),
            )
            .unwrap()
        })
    });

    #[cfg(feature = "parallel")]
    group.bench_function("parallel", |b| {
        b.iter(|| {
            minimize_parallel(
                black_box(rosenbrock),
                black_box(&[-5.0, -5.0]),
                black_box(&[5.0, 5.0]),
                config.clone(),
            )
            .unwrap()
        })
    });

    group.finish();
}

fn bench_griewank_10d(c: &mut Criterion) {
    let config = higher_d_config();
    let lower = [-600.0; 10];
    let upper = [600.0; 10];
    let mut group = c.benchmark_group("griewank_10d");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("serial", |b| {
        b.iter(|| {
            minimize(
                black_box(griewank_duan),
                black_box(&lower),
                black_box(&upper),
                config.clone(),
            )
            .unwrap()
        })
    });

    #[cfg(feature = "parallel")]
    group.bench_function("parallel", |b| {
        b.iter(|| {
            minimize_parallel(
                black_box(griewank_duan),
                black_box(&lower),
                black_box(&upper),
                config.clone(),
            )
            .unwrap()
        })
    });

    group.finish();
}

fn bench_shekel(c: &mut Criterion) {
    let config = higher_d_config();
    let mut group = c.benchmark_group("shekel");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("serial", |b| {
        b.iter(|| {
            minimize(
                black_box(shekel),
                black_box(&[0.0, 0.0, 0.0, 0.0]),
                black_box(&[10.0, 10.0, 10.0, 10.0]),
                config.clone(),
            )
            .unwrap()
        })
    });

    #[cfg(feature = "parallel")]
    group.bench_function("parallel", |b| {
        b.iter(|| {
            minimize_parallel(
                black_box(shekel),
                black_box(&[0.0, 0.0, 0.0, 0.0]),
                black_box(&[10.0, 10.0, 10.0, 10.0]),
                config.clone(),
            )
            .unwrap()
        })
    });

    group.finish();
}

fn bench_hartman_6d(c: &mut Criterion) {
    let config = higher_d_config();
    let mut group = c.benchmark_group("hartman_6d");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("serial", |b| {
        b.iter(|| {
            minimize(
                black_box(hartman),
                black_box(&[0.0; 6]),
                black_box(&[1.0; 6]),
                config.clone(),
            )
            .unwrap()
        })
    });

    #[cfg(feature = "parallel")]
    group.bench_function("parallel", |b| {
        b.iter(|| {
            minimize_parallel(
                black_box(hartman),
                black_box(&[0.0; 6]),
                black_box(&[1.0; 6]),
                config.clone(),
            )
            .unwrap()
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_goldstein_price,
    bench_rosenbrock,
    bench_griewank_10d,
    bench_shekel,
    bench_hartman_6d
);
criterion_main!(benches);
