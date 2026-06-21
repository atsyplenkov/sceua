<h1 align=center><code>sceua</code></h1>

<p align="center">
    <a href="https://github.com/atsyplenkov/sceua/releases">
        <img src="https://img.shields.io/github/v/release/atsyplenkov/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub&logoColor=white"></a>
    <a href="https://crates.io/crates/sceua/">
        <img src="https://img.shields.io/crates/v/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <a href="https://codecov.io/gh/atsyplenkov/sceua">
        <img src="https://img.shields.io/codecov/c/gh/atsyplenkov/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Codecov&logoColor=white"></a>
    <br>
    <a href="https://github.com/atsyplenkov/sceua/actions/workflows/ci.yaml">
        <img src="https://img.shields.io/github/actions/workflow/status/atsyplenkov/sceua/ci.yaml?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <a href="https://github.com/atsyplenkov/sceua/actions/workflows/R-CMD-check.yaml">
        <img src="https://img.shields.io/github/actions/workflow/status/atsyplenkov/sceua/R-CMD-check.yaml?style=flat&labelColor=1C2C2E&color=276DC3&logo=R&logoColor=white"></a>
    <!--<a href="https://docs.rs/sceua/">
        <img src="https://img.shields.io/docsrs/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>-->
    <br>
</p>

<h4 align="center">
  <a href="https://github.com/atsyplenkov/sceua/tree/main/rust">Rust crate</a> |
  <a href="https://github.com/atsyplenkov/sceua/tree/main/r">R package</a>
</h4>

A Rust implementation and R bindings of **SCE-UA**, the Shuffled Complex
Evolution algorithm for global optimisation developed by Duan et al.
([1992](https://onlinelibrary.wiley.com/doi/abs/10.1029/91WR02985)).

SCE-UA combines deterministic simplex search, competitive evolution, and periodic
shuffling of parallel complexes to solve nonlinear, non-convex, continuous
parameter estimation problems. It was originally developed for hydrologic model
calibration and remains widely used in hydrology and environmental modelling.

This crate is a complete rewrite in Rust of the original SCE-UA implementation in [Matlab](https://www.mathworks.com/matlabcentral/fileexchange/7671-shuffled-complex-evolution-sce-ua-method) and Fortran, as found in Qingyun Duan's thesis [Appendix I](https://repository.arizona.edu/handle/10150/185655).

## Rust

The Rust crate provides a fast, dependency-light SCE-UA implementation.

```rust
use sceua::{minimize, Config};

let result = minimize(
    |x| x.iter().map(|v| v * v).sum::<f64>(),
    &[-5.0, -5.0],
    &[5.0, 5.0],
    Config::default(),
)?;
```

See [`rust/README.md`](rust/README.md) for details.

## R

The R package exposes SCE-UA as a conventional optimiser via the `rextendr` framework:

```r
library(sceua)
set.seed(1969)
result <- sceua(
  fn = function(x) sum(x^2),
  lower = c(-5, -5),
  upper = c(5, 5),
  max_evaluations = 5000,
  kstop = 5,
  pcento = 1e-8,
  complexes = 5
)
```

Install from CRAN:

```r
# Not yet available
# install.packages("sceua")
```

Or install the development version from GitHub (requires a Rust toolchain):

```r
# install.packages("pak")
pak::pak("atsyplenkov/sceua/r")
```

See [`r/README.md`](r/README.md) for details.

## Statement of need

While working on a fork of the [`rtop`](https://cran.r-project.org/package=rtop) package (called `utop`), I found that the underlying R implementation of SCE-UA used for variogram fitting was a major performance bottleneck. Rewriting the algorithm in Rust with R bindings aims to provide a faster, safer, and more maintainable alternative for the R spatial-statistics community.

## Acknowledgements

Thanks to [Josiah Parry](https://github.com/JosiahParry) for setting up the `anime` monorepo as a template showing how to structure Rust-based multi-language bindings with `rextendr`.
