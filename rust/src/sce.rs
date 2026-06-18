use crate::{
    cce::evolve_simplex,
    config::{Config, ResolvedConfig},
    error::SceuaError,
    population::{
        compress_complexes, parameter_stats, random_point, sample_simplex_indices, sort_points,
        Point,
    },
    rng::DuanRng,
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct OptimizationResult {
    pub best_x: Vec<f64>,
    pub best_f: f64,
    pub evaluations: usize,
    pub loops: usize,
    pub termination: TerminationReason,
    pub history: Vec<HistoryEntry>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistoryEntry {
    pub loop_index: usize,
    pub evaluations: usize,
    pub complexes: usize,
    pub best_f: f64,
    pub worst_f: f64,
    pub geometric_range: f64,
    pub best_x: Vec<f64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminationReason {
    MaxEvaluations,
    ObjectiveConvergence,
    ParameterConvergence,
}

// SCEUA main routine.
// https://github.com/naddor/fuse/blob/e5fe0fbed82125eec4711854e1c5492da254df41/build/FUSE_SRC/FUSE_SCE/sce.f#L152-L399

pub fn minimize<F>(
    mut objective: F,
    lower: &[f64],
    upper: &[f64],
    config: Config,
) -> Result<OptimizationResult, SceuaError>
where
    F: FnMut(&[f64]) -> f64,
{
    validate_problem(lower, upper, &config)?;
    let resolved = config.resolve(lower.len())?;
    let mut rng = DuanRng::new(resolved.seed);
    let (population, evaluations) =
        initialize_population_serial(&mut objective, lower, upper, &config, resolved, &mut rng)?;
    continue_minimize(
        objective,
        lower,
        upper,
        resolved,
        population,
        evaluations,
        rng,
    )
}

#[cfg(feature = "parallel")]
pub fn minimize_parallel<F>(
    objective: F,
    lower: &[f64],
    upper: &[f64],
    config: Config,
) -> Result<OptimizationResult, SceuaError>
where
    F: Fn(&[f64]) -> f64 + Sync,
{
    validate_problem(lower, upper, &config)?;
    let resolved = config.resolve(lower.len())?;
    let mut rng = DuanRng::new(resolved.seed);
    let (population, evaluations) =
        initialize_population_parallel(&objective, lower, upper, &config, resolved, &mut rng)?;
    continue_minimize(
        objective,
        lower,
        upper,
        resolved,
        population,
        evaluations,
        rng,
    )
}

fn initialize_population_serial<F>(
    objective: &mut F,
    lower: &[f64],
    upper: &[f64],
    config: &Config,
    resolved: ResolvedConfig,
    rng: &mut DuanRng,
) -> Result<(Vec<Point>, usize), SceuaError>
where
    F: FnMut(&[f64]) -> f64,
{
    let target_population = resolved.complexes * resolved.points_per_complex;
    let mut evaluations = 0usize;
    let mut population = Vec::with_capacity(target_population);

    if config.include_initial {
        let initial = config
            .initial_point
            .as_deref()
            .ok_or(SceuaError::InvalidConfig(
                "include_initial requires initial_point",
            ))?;
        population.push(evaluate_point(objective, initial, &mut evaluations)?);
    }

    while population.len() < target_population && evaluations < resolved.max_evaluations {
        let point = random_point(lower, upper, rng);
        population.push(evaluate_point(objective, &point, &mut evaluations)?);
    }

    Ok((population, evaluations))
}

#[cfg(feature = "parallel")]
fn initialize_population_parallel<F>(
    objective: &F,
    lower: &[f64],
    upper: &[f64],
    config: &Config,
    resolved: ResolvedConfig,
    rng: &mut DuanRng,
) -> Result<(Vec<Point>, usize), SceuaError>
where
    F: Fn(&[f64]) -> f64 + Sync,
{
    let target_population = resolved.complexes * resolved.points_per_complex;
    let mut points = Vec::with_capacity(target_population.min(resolved.max_evaluations));

    if config.include_initial {
        let initial = config
            .initial_point
            .as_deref()
            .ok_or(SceuaError::InvalidConfig(
                "include_initial requires initial_point",
            ))?;
        points.push(initial.to_vec());
    }

    while points.len() < target_population && points.len() < resolved.max_evaluations {
        points.push(random_point(lower, upper, rng));
    }

    let evaluated: Vec<_> = points
        .par_iter()
        .map(|point| {
            let value = objective(point);
            if value.is_finite() {
                Ok(Point {
                    x: point.clone(),
                    value,
                })
            } else {
                Err(SceuaError::NonFiniteObjective { value })
            }
        })
        .collect();

    let evaluations = evaluated.len();
    let mut population = Vec::with_capacity(evaluations);
    for point in evaluated {
        population.push(point?);
    }
    Ok((population, evaluations))
}

fn continue_minimize<F>(
    mut objective: F,
    lower: &[f64],
    upper: &[f64],
    resolved: ResolvedConfig,
    mut population: Vec<Point>,
    evaluations: usize,
    mut rng: DuanRng,
) -> Result<OptimizationResult, SceuaError>
where
    F: FnMut(&[f64]) -> f64,
{
    sort_points(&mut population);
    let mut current_complexes = resolved.complexes;
    let mut evaluations = evaluations;
    let mut history = Vec::new();
    push_history(
        &mut history,
        0,
        evaluations,
        current_complexes,
        &population,
        lower,
        upper,
    );

    if evaluations >= resolved.max_evaluations {
        return Ok(result(
            population,
            evaluations,
            0,
            TerminationReason::MaxEvaluations,
            history,
        ));
    }

    let mut current_stats = parameter_stats(&population, lower, upper);
    if current_stats.geometric_range <= resolved.parameter_epsilon {
        return Ok(result(
            population,
            evaluations,
            0,
            TerminationReason::ParameterConvergence,
            history,
        ));
    }

    let mut best_by_loop = Vec::new();
    let mut loops = 0usize;

    loop {
        loops += 1;

        for complex_index in 0..current_complexes {
            let mut complex = partition_complex(
                &population,
                complex_index,
                current_complexes,
                resolved.points_per_complex,
            );

            for _ in 0..resolved.evolution_steps {
                if evaluations >= resolved.max_evaluations {
                    break;
                }

                let simplex_indices = sample_simplex_indices(
                    resolved.points_per_complex,
                    resolved.simplex_size,
                    &mut rng,
                );
                let mut simplex: Vec<_> = simplex_indices
                    .iter()
                    .map(|&index| complex[index].clone())
                    .collect();

                evolve_simplex(
                    &mut simplex,
                    lower,
                    upper,
                    &current_stats.normalized_std,
                    &mut rng,
                    &mut evaluations,
                    resolved.max_evaluations,
                    &mut objective,
                )?;

                for (&complex_position, point) in simplex_indices.iter().zip(simplex) {
                    complex[complex_position] = point;
                }
                sort_points(&mut complex);
            }

            replace_complex(
                &mut population,
                &complex,
                complex_index,
                current_complexes,
                resolved.points_per_complex,
            );

            if evaluations >= resolved.max_evaluations {
                break;
            }
        }

        sort_points(&mut population);
        push_history(
            &mut history,
            loops,
            evaluations,
            current_complexes,
            &population,
            lower,
            upper,
        );
        best_by_loop.push(population[0].value);

        if evaluations >= resolved.max_evaluations {
            return Ok(result(
                population,
                evaluations,
                loops,
                TerminationReason::MaxEvaluations,
                history,
            ));
        }

        if best_by_loop.len() > resolved.kstop {
            let current = *best_by_loop.last().expect("best_by_loop is not empty");
            let old = best_by_loop[best_by_loop.len() - resolved.kstop - 1];
            let denominator = (old + current).abs() / 2.0;
            let timeout = if denominator == 0.0 {
                if old == current {
                    0.0
                } else {
                    f64::INFINITY
                }
            } else {
                (old - current).abs() / denominator
            };
            if timeout < resolved.pcento {
                return Ok(result(
                    population,
                    evaluations,
                    loops,
                    TerminationReason::ObjectiveConvergence,
                    history,
                ));
            }
        }

        let next_stats = parameter_stats(&population, lower, upper);
        if next_stats.geometric_range <= resolved.parameter_epsilon {
            return Ok(result(
                population,
                evaluations,
                loops,
                TerminationReason::ParameterConvergence,
                history,
            ));
        }

        current_stats = next_stats;
        if current_complexes > resolved.min_complexes {
            let reduced = current_complexes - 1;
            population = compress_complexes(
                &population,
                current_complexes,
                reduced,
                resolved.points_per_complex,
            );
            current_complexes = reduced;
        }
    }
}

fn validate_problem(lower: &[f64], upper: &[f64], config: &Config) -> Result<(), SceuaError> {
    if lower.len() != upper.len() {
        return Err(SceuaError::BoundsLengthMismatch {
            lower: lower.len(),
            upper: upper.len(),
        });
    }
    if lower.is_empty() {
        return Err(SceuaError::EmptyProblem);
    }
    for (index, (&lo, &hi)) in lower.iter().zip(upper).enumerate() {
        if !lo.is_finite() || !hi.is_finite() || hi <= lo {
            return Err(SceuaError::InvalidBounds {
                index,
                lower: lo,
                upper: hi,
            });
        }
    }
    if let Some(initial) = &config.initial_point {
        if initial.len() != lower.len() {
            return Err(SceuaError::InitialPointLengthMismatch {
                expected: lower.len(),
                actual: initial.len(),
            });
        }
        for (index, ((&value, &lo), &hi)) in initial.iter().zip(lower).zip(upper).enumerate() {
            if !value.is_finite() || value < lo || value > hi {
                return Err(SceuaError::InvalidBounds {
                    index,
                    lower: lo,
                    upper: hi,
                });
            }
        }
    }
    Ok(())
}

fn evaluate_point<F>(
    objective: &mut F,
    point: &[f64],
    evaluations: &mut usize,
) -> Result<Point, SceuaError>
where
    F: FnMut(&[f64]) -> f64,
{
    let value = objective(point);
    *evaluations += 1;
    if value.is_finite() {
        Ok(Point {
            x: point.to_vec(),
            value,
        })
    } else {
        Err(SceuaError::NonFiniteObjective { value })
    }
}

fn partition_complex(
    population: &[Point],
    complex_index: usize,
    complexes: usize,
    points_per_complex: usize,
) -> Vec<Point> {
    (0..points_per_complex)
        .map(|point_index| population[point_index * complexes + complex_index].clone())
        .collect()
}

fn replace_complex(
    population: &mut [Point],
    complex: &[Point],
    complex_index: usize,
    complexes: usize,
    points_per_complex: usize,
) {
    for point_index in 0..points_per_complex {
        population[point_index * complexes + complex_index] = complex[point_index].clone();
    }
}

fn push_history(
    history: &mut Vec<HistoryEntry>,
    loop_index: usize,
    evaluations: usize,
    complexes: usize,
    population: &[Point],
    lower: &[f64],
    upper: &[f64],
) {
    let stats = parameter_stats(population, lower, upper);
    let best = &population[0];
    let worst = population.last().expect("population is not empty");
    history.push(HistoryEntry {
        loop_index,
        evaluations,
        complexes,
        best_f: best.value,
        worst_f: worst.value,
        geometric_range: stats.geometric_range,
        best_x: best.x.clone(),
    });
}

fn result(
    population: Vec<Point>,
    evaluations: usize,
    loops: usize,
    termination: TerminationReason,
    history: Vec<HistoryEntry>,
) -> OptimizationResult {
    let best = &population[0];
    OptimizationResult {
        best_x: best.x.clone(),
        best_f: best.value,
        evaluations,
        loops,
        termination,
        history,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimize_rejects_mismatched_bounds() {
        let err = minimize(|x| x[0], &[0.0], &[1.0, 2.0], Config::default()).unwrap_err();
        assert_eq!(err, SceuaError::BoundsLengthMismatch { lower: 1, upper: 2 });
    }

    // Full optimiser test identical to the Fortran version.
    // https://github.com/naddor/fuse/blob/e5fe0fbed82125eec4711854e1c5492da254df41/build/FUSE_SRC/FUSE_SCE/sce.f#L152-L399

    #[test]
    fn minimize_converges_on_two_dimensional_sphere() {
        let config = Config {
            max_evaluations: 5_000,
            kstop: 5,
            pcento: 1.0e-8,
            seed: 1969,
            complexes: 5,
            ..Config::default()
        };
        let result = minimize(
            |x| x.iter().map(|value| value * value).sum::<f64>(),
            &[-5.0, -5.0],
            &[5.0, 5.0],
            config,
        )
        .unwrap();

        assert!(result.best_f < 1.0e-6, "{result:?}");
        assert!(matches!(
            result.termination,
            TerminationReason::ObjectiveConvergence
                | TerminationReason::ParameterConvergence
                | TerminationReason::MaxEvaluations
        ));
    }

    // Parallel initialization should preserve the serial trajectory after initial sampling.
    // Serial trajectory source: https://github.com/naddor/fuse/blob/e5fe0fbed82125eec4711854e1c5492da254df41/build/FUSE_SRC/FUSE_SCE/sce.f#L152-L399

    #[cfg(feature = "parallel")]
    #[test]
    fn parallel_minimize_matches_serial_for_pure_objective() {
        let config = Config {
            max_evaluations: 5_000,
            kstop: 5,
            pcento: 1.0e-8,
            seed: 1969,
            complexes: 5,
            ..Config::default()
        };
        let objective = |x: &[f64]| x.iter().map(|value| value * value).sum::<f64>();

        let serial = minimize(objective, &[-5.0, -5.0], &[5.0, 5.0], config.clone()).unwrap();
        let parallel = minimize_parallel(objective, &[-5.0, -5.0], &[5.0, 5.0], config).unwrap();

        assert_eq!(serial.best_x, parallel.best_x);
        assert_eq!(serial.best_f, parallel.best_f);
        assert_eq!(serial.evaluations, parallel.evaluations);
        assert_eq!(serial.termination, parallel.termination);
    }
}
