<h1 align=center><code>sceua</code></h1>

<p align="center">
    <a href="https://github.com/atsyplenkov/sceua/releases">
        <img src="https://img.shields.io/github/v/release/atsyplenkov/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub&logoColor=white"></a>
    <a href="https://crates.io/crates/sceua/">
        <img src="https://img.shields.io/crates/v/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <a href="https://codecov.io/gh/atsyplenkov/sceua">
        <img src="https://img.shields.io/codecov/c/gh/atsyplenkov/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Codecov&logoColor=white"></a>
    <br>
    <a href="https://github.com/atsyplenkov/sceua/actions/workflows/ci.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/atsyplenkov/sceua/ci.yml?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub%20Actions&logoColor=white"></a>
    <a href="https://docs.rs/sceua/">
        <img src="https://img.shields.io/docsrs/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <br>
</p>

<h4 align="center">
  <a href="https://github.com/atsyplenkov/sceua/tree/main/rust">Rust crate</a> |
  <a href="https://github.com/atsyplenkov/sceua/tree/main/r">R package</a>
</h4>

A Rust implementation and R bindings of **SCE-UA**, the Shuffled Complex Evolution algorithm for global optimization developed by Duan et al. ([1992](https://onlinelibrary.wiley.com/doi/abs/10.1029/91WR02985)).

SCE-UA combines deterministic simplex search, competitive evolution, and periodic shuffling of parallel complexes to robustly solve nonlinear, non-convex optimization problems. It was originally developed for hydrologic model calibration but is broadly applicable to any continuous parameter estimation task.

This crate is a complete rewrite of the original SCE-UA implementation in [Matlab](https://www.mathworks.com/matlabcentral/fileexchange/7671-shuffled-complex-evolution-sce-ua-method) and Fortran, as found in Qingyun Duan's thesis [Appendix I](https://repository.arizona.edu/handle/10150/185655).

> **Status**: This project is currently a fork of [josiahparry/anime](https://github.com/JosiahParry/anime), used as a monorepo template for Rust+R bindings. The SCE-UA algorithm is planned but not yet implemented.

## Rust

The Rust crate will provide a fast, pure-Rust implementation of the SCE-UA algorithm with the following features:

- `sceua::minimize(f, lower, upper, ...)` — minimize an arbitrary objective function over bounded parameter ranges
- Configurable algorithmic parameters (number of complexes, population size, convergence criteria)
- `no_std`-compatible core with optional `rayon`-based parallelism

```rust
// Planned API sketch
use sceua::{Config, minimize};

let config = Config::default();
let (x_opt, f_opt) = minimize(rosenbrock, &[(-5.0, 5.0); 10], &config)?;
```

## R

The R package will expose SCE-UA as a drop-in optimizer via the `rextendr` framework:

```r
library(sceua)

# Minimize the Rosenbrock function
result <- sceua(
  fn  = function(x) { (1 - x[1])^2 + 100 * (x[2] - x[1]^2)^2 },
  lower = c(-5, -5),
  upper = c(5, 5)
)
```

## Statement of need

While working on a fork of the [`rtop`](https://cran.r-project.org/package=rtop) package (called `utop`), I found that the underlying R implementation of SCE-UA used for variogram fitting was a major performance bottleneck. Rewriting the algorithm in Rust with R bindings aims to provide a faster, safer, and more maintainable alternative for the R spatial-statistics community.

## Acknowledgements

Thanks to [Josiah Parry](https://github.com/JosiahParry) for setting up the `anime` monorepo as a template showing how to structure Rust-based multi-language bindings with `rextendr`.
