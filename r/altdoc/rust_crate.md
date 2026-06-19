# sceua

A Rust implementation of **SCE-UA**, the Shuffled Complex Evolution algorithm for
global optimisation developed by Duan et al. (1992).

SCE-UA combines deterministic simplex search, competitive evolution, and periodic
shuffling of parallel complexes. It was originally developed for hydrologic
model calibration and is broadly applicable to continuous, bounded nonlinear
optimisation problems.

This crate is a complete rewrite in Rust of the original SCE-UA implementation in [Matlab](https://www.mathworks.com/matlabcentral/fileexchange/7671-shuffled-complex-evolution-sce-ua-method) and Fortran, as found in Qingyun Duan's thesis [Appendix I](https://repository.arizona.edu/handle/10150/185655).

## Usage

```rust
use sceua::{minimize, Config};

let result = minimize(
    |x| x.iter().map(|v| v * v).sum::<f64>(),
    &[-5.0, -5.0],
    &[5.0, 5.0],
    Config::default(),
)?;

println!("best = {:?}", result.best_x);
println!("value = {}", result.best_f);
```

## Features

- `minimize(f, lower, upper, config)` — serial optimisation of a bounded
  objective function.
- `minimize_parallel(f, lower, upper, config)` — parallel evaluation of the
  initial population when the `parallel` feature is enabled.
- Configurable population geometry, convergence criteria, and initial point.
- Deterministic, Fortran-compatible RAN1 random number generator for reproducible
  results.

## Configuration

`Config::default()` provides the following defaults:

| Field | Default | Description |
|---|---|---|
| `max_evaluations` | `10000` | Maximum objective evaluations |
| `kstop` | `5` | Shuffling loops for convergence test |
| `pcento` | `0.01` | Objective convergence threshold (%) |
| `seed` | `1969` | Random seed |
| `complexes` | `2` | Initial number of complexes |
| `points_per_complex` | `2*n + 1` | Points per complex |
| `simplex_size` | `n + 1` | Points per sub-complex |
| `evolution_steps` | `points_per_complex` | Evolution steps before shuffling |
| `min_complexes` | `complexes` | Minimum complexes after reduction |
| `parameter_epsilon` | `1e-3` | Parameter-space convergence threshold |

Here `n` is the number of parameters (the length of `lower`/`upper`).

## Parallel evaluation

Enable the `parallel` feature to evaluate the initial population in parallel:

```toml
[dependencies]
sceua = { version = "0.0.1", features = ["parallel"] }
```

```rust
use sceua::{minimize_parallel, Config};

let result = minimize_parallel(
    |x| x.iter().map(|v| v * v).sum::<f64>(),
    &[-5.0, -5.0, -5.0],
    &[5.0, 5.0, 5.0],
    Config::default(),
)?;
```

## Return value

`minimize` returns an `OptimizationResult` containing:

- `best_x`: best parameter vector found.
- `best_f`: objective value at `best_x`.
- `evaluations`: number of objective evaluations used.
- `loops`: number of shuffling loops completed.
- `termination`: reason for termination (`MaxEvaluations`,
  `ObjectiveConvergence`, or `ParameterConvergence`).
- `history`: per-loop history of best/worst objective values, geometric range,
  and population metrics.

## Development

Run the test suite:

```sh
cargo test -p sceua --locked
cargo test -p sceua --all-features --locked
```

## Benchmarks

Criterion benchmarks comparing serial and parallel execution on Duan's test
problems live in `benches/benchmark.rs`:

```sh
# Serial only
cargo bench -p sceua

# Serial vs parallel
cargo bench -p sceua --features parallel
```

The `parallel` feature only parallelises the initial population evaluation, so
the largest speed-ups appear for expensive objective functions or higher-
dimensional problems with larger initial populations.

## References

Duan, Q., Sorooshian, S., and Gupta, V.K., 1992. Effective and efficient
global optimization for conceptual rainfall-runoff models.
*Water Resources Research* 28 (4), 1015-1031.
