# sceua

<!-- badges: start -->
<!-- badges: end -->

`sceua` provides R bindings to a Rust implementation of the Shuffled Complex
Evolution - University of Arizona (SCE-UA) global optimisation algorithm
(Duan et al., 1992).

## Installation

You need a Rust toolchain installed. Then install the development version
from GitHub:

``` r
# install.packages("pak")
pak::pak("atsyplenkov/sceua/r")
```

## Example

``` r
library(sceua)

result <- sceua(
  fn = function(x) sum(x^2),
  lower = c(-5, -5),
  upper = c(5, 5),
  max_evaluations = 5000,
  kstop = 5,
  pcento = 1e-8,
  seed = 1969,
  complexes = 5
)

result
```

## References

Duan, Q., Sorooshian, S., and Gupta, V.K., 1992. Effective and efficient
global optimization for conceptual rainfall-runoff models.
*Water Resources Research* 28 (4), 1015-1031.
