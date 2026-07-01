# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-07-01

### Changed

- Lower the CRAN-facing Rust requirement to the verified MSRV, Rust 1.81.

## [0.1.0] - 2026-06-21

### Added

- Initial Rust implementation of the SCE-UA global optimiser.
- `minimize(f, lower, upper, config)` entry point with configurable population
  geometry, convergence criteria, and initial point.
- Deterministic, Fortran-compatible RAN1 random number generator for reproducible
  results.
- `OptimizationResult` with per-loop history and termination reason.
- Seven Duan test functions (`duan_test_func`) for benchmarking and validation.
