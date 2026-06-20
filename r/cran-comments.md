## Test environments

* local Arch Linux, R 4.6.0 (release)
* GitHub Actions: macOS-latest, windows-latest, ubuntu-latest (release, oldrel-1)
  via `r-lib/actions/check-r-package@v2` with `--no-manual`

## R CMD check results

0 errors | 1 warning | 2 notes

### WARNING: compiled code calls entry points which might terminate R

`R CMD check` reports `_exit`, `abort`, and `exit` in `rust/target/release/libsceua.a`:

```
Found '_exit', possibly from '_exit' (C)
  Object: 'rust/target/release/libsceua.a'
Found 'abort', possibly from 'abort' (C)
  Object: 'rust/target/release/libsceua.a'
Found 'exit', possibly from 'exit' (C)
  Object: 'rust/target/release/libsceua.a'
```

These symbols originate from the Rust standard library's panic and
process-abort runtime support pulled in by `extendr-api`, not from
package code. The package never calls these entry points directly; the
extendr panic hook installed in `src/entrypoint.c`
(`register_extendr_panic_hook()`) routes Rust panics to R's error
handler rather than terminating the process. This warning is a known
false positive shared by other extendr-based packages on CRAN.

### NOTE: New submission

This is the first submission of `sceua`.

### NOTE: installed package size / tarball size

The source tarball is approximately 0.8 MB. Rust dependencies are
vendored into `src/rust/vendor.tar.xz` so that the package builds
offline, as required by CRAN. The vendored archive accounts for the
bulk of the tarball size.

## Dependencies

Rust dependencies are vendored via `rextendr::vendor_crates()` and
built offline with `cargo build --offline`. No Rust crates are
downloaded during installation.

## MSRV

Minimum supported Rust version is 1.91.1, declared in `DESCRIPTION`
(`SystemRequirements`) and enforced by `configure` via `tools/msrv.R`.
