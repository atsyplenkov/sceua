use crate::{minimize, Config};

fn goldstein_price(x: &[f64]) -> f64 {
    let x1 = x[0];
    let x2 = x[1];
    let u1 = (x1 + x2 + 1.0).powi(2);
    let u2 = 19.0 - 14.0 * x1 + 3.0 * x1.powi(2) - 14.0 * x2 + 6.0 * x1 * x2 + 3.0 * x2.powi(2);
    let u3 = (2.0 * x1 - 3.0 * x2).powi(2);
    let u4 = 18.0 - 32.0 * x1 + 12.0 * x1.powi(2) + 48.0 * x2 - 36.0 * x1 * x2 + 27.0 * x2.powi(2);
    (1.0 + u1 * u2) * (30.0 + u3 * u4)
}

fn rosenbrock(x: &[f64]) -> f64 {
    100.0 * (x[1] - x[0].powi(2)).powi(2) + (1.0 - x[0]).powi(2)
}

fn six_hump_camelback(x: &[f64]) -> f64 {
    let x1 = x[0];
    let x2 = x[1];
    (4.0 - 2.1 * x1.powi(2) + x1.powi(4) / 3.0) * x1.powi(2)
        + x1 * x2
        + (-4.0 + 4.0 * x2.powi(2)) * x2.powi(2)
}

fn rastrigin_duan(x: &[f64]) -> f64 {
    x[0].powi(2) + x[1].powi(2) - (18.0 * x[0]).cos() - (18.0 * x[1]).cos()
}

fn griewank_duan(x: &[f64]) -> f64 {
    let divisor = if x.len() == 2 { 200.0 } else { 4000.0 };
    let sum = x.iter().map(|value| value.powi(2) / divisor).sum::<f64>();
    let product = x
        .iter()
        .enumerate()
        .map(|(index, value)| (value / ((index + 1) as f64).sqrt()).cos())
        .product::<f64>();
    sum - product + 1.0
}

fn shekel(x: &[f64]) -> f64 {
    let a = [
        [4.0, 1.0, 8.0, 6.0, 3.0, 2.0, 5.0, 8.0, 6.0, 7.0],
        [4.0, 1.0, 8.0, 6.0, 7.0, 9.0, 5.0, 1.0, 2.0, 3.6],
        [4.0, 1.0, 8.0, 6.0, 3.0, 2.0, 3.0, 8.0, 6.0, 7.0],
        [4.0, 1.0, 8.0, 6.0, 7.0, 9.0, 3.0, 1.0, 2.0, 3.6],
    ];
    let c = [0.1, 0.2, 0.2, 0.4, 0.4, 0.6, 0.3, 0.7, 0.5, 0.5];
    let mut f = 0.0;
    for i in 0..10 {
        let mut u = 0.0;
        for j in 0..x.len() {
            u += (x[j] - a[j][i]).powi(2);
        }
        f -= 1.0 / (u + c[i]);
    }
    f
}

fn hartman(x: &[f64]) -> f64 {
    let a6 = [
        [10.0, 0.05, 3.0, 17.0],
        [3.0, 10.0, 3.5, 8.0],
        [17.0, 17.0, 1.7, 0.05],
        [3.5, 0.1, 10.0, 10.0],
        [1.7, 8.0, 17.0, 0.1],
        [8.0, 14.0, 8.0, 14.0],
    ];
    let c6 = [1.0, 1.2, 3.0, 3.2];
    let p6 = [
        [0.1312, 0.2329, 0.2348, 0.4047],
        [0.1696, 0.4135, 0.1451, 0.8828],
        [0.5569, 0.8307, 0.3522, 0.8732],
        [0.0124, 0.3736, 0.2883, 0.5743],
        [0.8283, 0.1004, 0.3047, 0.1091],
        [0.5886, 0.9991, 0.6650, 0.0381],
    ];

    let a3 = [
        [3.0, 0.1, 3.0, 0.1],
        [10.0, 10.0, 10.0, 10.0],
        [30.0, 35.0, 30.0, 35.0],
    ];
    let c3 = [1.0, 1.2, 3.0, 3.2];
    let p3 = [
        [0.3689, 0.4699, 0.1091, 0.03815],
        [0.1170, 0.4387, 0.8732, 0.5743],
        [0.2673, 0.7470, 0.5547, 0.8828],
    ];

    let mut f = 0.0;
    for i in 0..4 {
        let mut u = 0.0;
        for j in 0..x.len() {
            let a = if x.len() == 3 { a3[j][i] } else { a6[j][i] };
            let p = if x.len() == 3 { p3[j][i] } else { p6[j][i] };
            u += a * (x[j] - p).powi(2);
        }
        let c = if x.len() == 3 { c3[i] } else { c6[i] };
        f -= c * (-u).exp();
    }
    f
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "actual={actual}, expected={expected}, tolerance={tolerance}"
    );
}

#[test]
fn duan_test_functions_match_documented_optima() {
    assert_close(goldstein_price(&[0.0, -1.0]), 3.0, 1.0e-12);
    assert_close(rosenbrock(&[1.0, 1.0]), 0.0, 1.0e-12);
    assert_close(
        six_hump_camelback(&[0.08983, -0.7126]),
        -1.0316284229280819,
        1.0e-8,
    );
    assert_close(rastrigin_duan(&[0.0, 0.0]), -2.0, 1.0e-12);
    assert_close(griewank_duan(&[0.0, 0.0]), 0.0, 1.0e-12);
    assert_close(griewank_duan(&[0.0; 10]), 0.0, 1.0e-12);
    assert_close(shekel(&[4.0, 4.0, 4.0, 4.0]), -10.536283726219603, 1.0e-12);
    assert_close(
        hartman(&[0.201, 0.150, 0.477, 0.275, 0.311, 0.657]),
        -3.3223349676854577,
        1.0e-12,
    );
}

#[test]
fn sceua_minimizes_goldstein_price_with_duan_bounds() {
    let config = Config {
        max_evaluations: 10_000,
        kstop: 5,
        pcento: 0.01,
        seed: 1969,
        complexes: 5,
        ..Config::default()
    };
    let result = minimize(goldstein_price, &[-2.0, -2.0], &[2.0, 2.0], config).unwrap();

    assert!(result.best_f <= 3.001, "{result:?}");
    assert!((result.best_x[0] - 0.0).abs() <= 0.01, "{result:?}");
    assert!((result.best_x[1] + 1.0).abs() <= 0.01, "{result:?}");
}

#[test]
fn sceua_minimizes_rosenbrock_with_duan_bounds() {
    let config = Config {
        max_evaluations: 20_000,
        kstop: 5,
        pcento: 0.0,
        seed: 1969,
        complexes: 10,
        ..Config::default()
    };
    let result = minimize(rosenbrock, &[-5.0, -5.0], &[5.0, 5.0], config).unwrap();

    assert!(result.best_f <= 1.0e-3, "{result:?}");
    assert!((result.best_x[0] - 1.0).abs() <= 0.05, "{result:?}");
    assert!((result.best_x[1] - 1.0).abs() <= 0.05, "{result:?}");
}
