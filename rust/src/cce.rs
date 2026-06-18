use crate::{error::SceuaError, population::Point, rng::DuanRng};

// CCE subroutine.
// reflection -> contraction -> Gaussian mutation sequence
// See
// https://github.com/naddor/fuse/blob/e5fe0fbed82125eec4711854e1c5492da254df41/build/FUSE_SRC/FUSE_SCE/sce.f#L431-L546

pub(crate) fn evolve_simplex<F>(
    simplex: &mut [Point],
    lower: &[f64],
    upper: &[f64],
    normalized_std: &[f64],
    rng: &mut DuanRng,
    evaluations: &mut usize,
    max_evaluations: usize,
    objective: &mut F,
) -> Result<(), SceuaError>
where
    F: FnMut(&[f64]) -> f64,
{
    if simplex.is_empty() || *evaluations >= max_evaluations {
        return Ok(());
    }

    let worst_index = simplex.len() - 1;
    let dimension = lower.len();
    let worst = simplex[worst_index].x.clone();
    let worst_value = simplex[worst_index].value;
    let centroid = centroid_without_worst(simplex, dimension);
    let step: Vec<_> = centroid
        .iter()
        .zip(&worst)
        .map(|(&ce, &wo)| ce - wo)
        .collect();

    let reflected: Vec<_> = worst
        .iter()
        .zip(&step)
        .map(|(&wo, &step)| wo + 2.0 * step)
        .collect();
    let mut trial = if within_bounds(&reflected, lower, upper) {
        reflected
    } else {
        gaussian_point(simplex, lower, upper, normalized_std, rng)
    };

    let mut trial_value = evaluate(objective, &trial, evaluations)?;
    if trial_value <= worst_value {
        simplex[worst_index] = Point {
            x: trial,
            value: trial_value,
        };
        return Ok(());
    }
    if *evaluations >= max_evaluations {
        return Ok(());
    }

    trial = worst
        .iter()
        .zip(&step)
        .map(|(&wo, &step)| wo + 0.5 * step)
        .collect();
    trial_value = evaluate(objective, &trial, evaluations)?;
    if trial_value <= worst_value {
        simplex[worst_index] = Point {
            x: trial,
            value: trial_value,
        };
        return Ok(());
    }
    if *evaluations >= max_evaluations {
        return Ok(());
    }

    trial = gaussian_point(simplex, lower, upper, normalized_std, rng);
    trial_value = evaluate(objective, &trial, evaluations)?;
    simplex[worst_index] = Point {
        x: trial,
        value: trial_value,
    };
    Ok(())
}

fn centroid_without_worst(simplex: &[Point], dimension: usize) -> Vec<f64> {
    let divisor = (simplex.len() - 1) as f64;
    let mut centroid = vec![0.0; dimension];
    for point in &simplex[..simplex.len() - 1] {
        for (sum, value) in centroid.iter_mut().zip(&point.x) {
            *sum += *value;
        }
    }
    for value in &mut centroid {
        *value /= divisor;
    }
    centroid
}

fn gaussian_point(
    simplex: &[Point],
    lower: &[f64],
    upper: &[f64],
    normalized_std: &[f64],
    rng: &mut DuanRng,
) -> Vec<f64> {
    let best = &simplex[0].x;
    let mut point = Vec::with_capacity(best.len());
    for parameter in 0..best.len() {
        let bound = upper[parameter] - lower[parameter];
        loop {
            let candidate = best[parameter] + normalized_std[parameter] * rng.gaussian() * bound;
            if candidate >= lower[parameter] && candidate <= upper[parameter] {
                point.push(candidate);
                break;
            }
        }
    }
    point
}

fn within_bounds(point: &[f64], lower: &[f64], upper: &[f64]) -> bool {
    point
        .iter()
        .zip(lower.iter().zip(upper))
        .all(|(&value, (&lo, &hi))| value >= lo && value <= hi)
}

fn evaluate<F>(objective: &mut F, point: &[f64], evaluations: &mut usize) -> Result<f64, SceuaError>
where
    F: FnMut(&[f64]) -> f64,
{
    let value = objective(point);
    *evaluations += 1;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SceuaError::NonFiniteObjective { value })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f64, value: f64) -> Point {
        Point { x: vec![x], value }
    }

    #[test]
    fn cce_accepts_reflection_when_it_improves_worst_point() {
        let mut simplex = vec![point(0.0, 0.0), point(1.0, 1.0), point(2.0, 4.0)];
        let mut rng = DuanRng::new(1969);
        let mut evaluations = 0;
        let mut objective = |x: &[f64]| x[0] * x[0];

        evolve_simplex(
            &mut simplex,
            &[-10.0],
            &[10.0],
            &[0.1],
            &mut rng,
            &mut evaluations,
            10,
            &mut objective,
        )
        .unwrap();

        assert_eq!(evaluations, 1);
        assert!((simplex[2].x[0] + 1.0).abs() < 1.0e-12);
        assert!((simplex[2].value - 1.0).abs() < 1.0e-12);
    }

    #[test]
    fn cce_tries_contraction_after_failed_reflection() {
        let mut simplex = vec![point(0.0, 0.0), point(1.0, 0.5), point(2.0, 1.0)];
        let mut rng = DuanRng::new(1969);
        let mut evaluations = 0;
        let mut objective = |x: &[f64]| (x[0] - 1.0) * (x[0] - 1.0);

        evolve_simplex(
            &mut simplex,
            &[-10.0],
            &[10.0],
            &[0.1],
            &mut rng,
            &mut evaluations,
            10,
            &mut objective,
        )
        .unwrap();

        assert_eq!(evaluations, 2);
        assert!((simplex[2].x[0] - 1.25).abs() < 1.0e-12);
        assert!((simplex[2].value - 0.0625).abs() < 1.0e-12);
    }

    #[test]
    fn cce_uses_gaussian_mutation_when_reflection_is_out_of_bounds() {
        let mut simplex = vec![point(0.9, 0.0), point(0.1, 1.0)];
        let mut rng = DuanRng::new(1969);
        let mut evaluations = 0;
        let mut objective = |x: &[f64]| (x[0] - 0.9).abs();

        evolve_simplex(
            &mut simplex,
            &[0.0],
            &[1.0],
            &[0.05],
            &mut rng,
            &mut evaluations,
            10,
            &mut objective,
        )
        .unwrap();

        assert_eq!(evaluations, 1);
        assert!(simplex[1].x[0] >= 0.0 && simplex[1].x[0] <= 1.0);
        assert_ne!(simplex[1].x[0], 1.7);
    }

    #[test]
    fn cce_does_not_replace_worst_when_failed_reflection_hits_max_evaluations() {
        let mut simplex = vec![point(0.0, 0.0), point(1.0, 0.5), point(2.0, 1.0)];
        let mut rng = DuanRng::new(1969);
        let mut evaluations = 0;
        let mut objective = |_x: &[f64]| 2.0;

        evolve_simplex(
            &mut simplex,
            &[-10.0],
            &[10.0],
            &[0.1],
            &mut rng,
            &mut evaluations,
            1,
            &mut objective,
        )
        .unwrap();

        assert_eq!(evaluations, 1);
        assert_eq!(simplex[2], point(2.0, 1.0));
    }
}
