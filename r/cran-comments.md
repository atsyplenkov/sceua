## Test environments

* local Arch Linux, R 4.6.0 (release) with `--as-cran`
* Winbuilder (R-devel, Windows Server 2022 x64)
* GitHub Actions: macOS-latest, windows-latest, ubuntu-latest (release, oldrel-1)
  via `r-lib/actions/check-r-package@v2`

## R CMD check results

0 errors | 0 warnings | 1 note

### NOTE: New submission

This is the first submission of `sceua`.

CRAN incoming feasibility also flags `Duan`, `SCE`, `UA`, `et`, `al`,
and `hydrological` as possibly misspelled words in DESCRIPTION. These
are an author name, well-known acronyms, and correct English.

## Dependencies

Rust dependencies are vendored via `rextendr::vendor_crates()` and
built offline with `cargo build --offline`. No Rust crates are
downloaded during installation. The source tarball is approximately
0.55 MB. Development-only Rust dependencies and benchmarks are stripped
from the bootstrapped CRAN source before vendoring and remain available
in the repository's Rust crate.

## MSRV

Minimum supported Rust version is 1.91.1, declared in `DESCRIPTION`
(`SystemRequirements`) and enforced by `configure` via `tools/config.R`.
