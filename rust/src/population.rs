use crate::rng::DuanRng;

const DELTA: f64 = 1.0e-20;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Point {
    pub(crate) x: Vec<f64>,
    pub(crate) value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ParameterStats {
    pub(crate) normalized_std: Vec<f64>,
    pub(crate) geometric_range: f64,
}

pub(crate) fn sort_points(points: &mut [Point]) {
    points.sort_by(|left, right| left.value.total_cmp(&right.value));
}

pub(crate) fn random_point(lower: &[f64], upper: &[f64], rng: &mut DuanRng) -> Vec<f64> {
    lower
        .iter()
        .zip(upper)
        .map(|(&lo, &hi)| lo + (hi - lo) * rng.uniform())
        .collect()
}

pub(crate) fn parameter_stats(points: &[Point], lower: &[f64], upper: &[f64]) -> ParameterStats {
    let dimension = lower.len();
    let count = points.len() as f64;
    let mut normalized_std = vec![0.0; dimension];
    let mut log_range_sum = 0.0;

    for parameter in 0..dimension {
        let bound = upper[parameter] - lower[parameter];
        let mut min_value = f64::INFINITY;
        let mut max_value = f64::NEG_INFINITY;
        let mut sum = 0.0;
        let mut sum_squares = 0.0;

        for point in points {
            let value = point.x[parameter];
            min_value = min_value.min(value);
            max_value = max_value.max(value);
            sum += value;
            sum_squares += value * value;
        }

        let mean = sum / count;
        let mut variance = sum_squares / count - mean * mean;
        if variance <= DELTA {
            variance = DELTA;
        }
        normalized_std[parameter] = variance.sqrt() / bound;
        log_range_sum += (DELTA + (max_value - min_value) / bound).ln();
    }

    ParameterStats {
        normalized_std,
        geometric_range: (log_range_sum / dimension as f64).exp(),
    }
}

#[cfg(test)]
pub(crate) fn normalized_distances(
    points: &[Point],
    initial: &[f64],
    lower: &[f64],
    upper: &[f64],
) -> Vec<f64> {
    points
        .iter()
        .map(|point| {
            point
                .x
                .iter()
                .zip(initial)
                .zip(lower.iter().zip(upper))
                .map(|((&x, &xi), (&lo, &hi))| (x - xi).abs() / (hi - lo))
                .sum::<f64>()
                / initial.len() as f64
        })
        .collect()
}

pub(crate) fn sample_simplex_indices(
    points_per_complex: usize,
    simplex_size: usize,
    rng: &mut DuanRng,
) -> Vec<usize> {
    if simplex_size == points_per_complex {
        return (0..simplex_size).collect();
    }

    let mut indices = Vec::with_capacity(simplex_size);
    while indices.len() < simplex_size {
        let candidate = sample_rank(points_per_complex, rng);
        if !indices.contains(&candidate) {
            indices.push(candidate);
        }
    }
    indices.sort_unstable();
    indices
}

pub(crate) fn compress_complexes(
    population: &[Point],
    old_complexes: usize,
    new_complexes: usize,
    points_per_complex: usize,
) -> Vec<Point> {
    let mut compressed = Vec::with_capacity(new_complexes * points_per_complex);
    for point_index in 0..points_per_complex {
        for complex_index in 0..new_complexes {
            let old_index = point_index * old_complexes + complex_index;
            compressed.push(population[old_index].clone());
        }
    }
    compressed
}

fn sample_rank(points_per_complex: usize, rng: &mut DuanRng) -> usize {
    let npg = points_per_complex as f64;
    let random = rng.uniform();
    let one_based =
        1.0 + (npg + 0.5 - ((npg + 0.5).powi(2) - npg * (npg + 1.0) * random).sqrt()).trunc();
    one_based as usize - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: &[f64], value: f64) -> Point {
        Point {
            x: x.to_vec(),
            value,
        }
    }

    #[test]
    fn parstt_matches_duan_population_statistics() {
        let points = vec![
            point(&[0.0, 2.0], 0.0),
            point(&[2.0, 4.0], 0.0),
            point(&[4.0, 8.0], 0.0),
        ];
        let stats = parameter_stats(&points, &[0.0, 0.0], &[4.0, 8.0]);

        let expected_std = (8.0_f64 / 3.0).sqrt() / 4.0;
        assert!((stats.normalized_std[0] - expected_std).abs() < 1.0e-12);
        assert!((stats.normalized_std[1] - 0.31180478223116176).abs() < 1.0e-12);
        assert!((stats.geometric_range - 0.75_f64.sqrt()).abs() < 1.0e-12);
    }

    #[test]
    fn normdist_matches_duan_formula() {
        let points = vec![point(&[2.0, 6.0], 0.0), point(&[4.0, 0.0], 0.0)];
        let distances = normalized_distances(&points, &[0.0, 4.0], &[0.0, 0.0], &[4.0, 8.0]);
        assert!((distances[0] - 0.375).abs() < 1.0e-12);
        assert!((distances[1] - 0.75).abs() < 1.0e-12);
    }

    #[test]
    fn simplex_indices_follow_duan_linear_probability() {
        let mut rng = DuanRng::new(1969);
        let indices = sample_simplex_indices(5, 3, &mut rng);
        assert_eq!(indices, vec![0, 2, 3]);
    }

    #[test]
    fn comp_drops_lowest_ranked_complex() {
        let population: Vec<_> = (0..9).map(|i| point(&[i as f64], i as f64)).collect();
        let compressed = compress_complexes(&population, 3, 2, 3);
        let values: Vec<_> = compressed
            .iter()
            .map(|point| point.value as usize)
            .collect();
        assert_eq!(values, vec![0, 1, 3, 4, 6, 7]);
    }
}
