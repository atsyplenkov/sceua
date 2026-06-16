<h1 align=center><code>sceua</code></h1>

<p align="center">
    <a href="https://github.com/atsyplenkov//releases">
        <img src="https://img.shields.io/github/v/release/atsyplenkov/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub&logoColor=white"></a>
    <a href="https://crates.io/crates/sceua/">
        <img src="https://img.shields.io/crates/v/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <a href="https://codecov.io/gh/atsyplenkov/sceua">
        <img src="https://img.shields.io/codecov/c/gh/atsyplenkov/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Codecov&logoColor=white"></a>
    <br>
    <a href="https://github.com/atsyplenkov/sceua/actions/workflows/ci.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/atsyplenkov/sceua/ci.yml?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub%20Actions&logoColor=white"></a>
    <a href="https://github.com/atsyplenkov/sceua/actions/workflows/rust-cd.yml">
        <img src="https://img.shields.io/github/actions/workflow/status/atsyplenkov/sceua/rust-cd.yml?style=flat&labelColor=1C2C2E&color=dea584&logo=GitHub%20Actions&logoColor=white&label=deploy"></a>
    <a href="https://docs.rs/sceua/">
        <img src="https://img.shields.io/docsrs/sceua?style=flat&labelColor=1C2C2E&color=dea584&logo=Rust&logoColor=white"></a>
    <br>
</p>

<h4 align="center">
  <a href="https://github.com/atsyplenkov/sceua/tree/main/rust">Rust crate</a> |
  <a href="https://github.com/atsyplenkov/sceua/tree/main/r">R package</a>
</h4>


A Rust implementation and R bindings of **SCE-UA**, a Shuffle Complex Evolution Algorithm for Optimization by Duan et al. ([1992](https://onlinelibrary.wiley.com/doi/abs/10.1029/91WR02985)).

This crate is a complete rewrite of the original SCE-UA implementation in [Matlab](https://www.mathworks.com/matlabcentral/fileexchange/7671-shuffled-complex-evolution-sce-ua-method) and Fortran as found in Qingyun Duan's thesis [Appendix I](https://repository.arizona.edu/handle/10150/185655).

# Rust
`TBA`

# R
`TBA`

## Statement of the need
I've been working recently on the `rtop` fork called `utop` which uses SCE-UA for variogram fitting. After running an R profiling I found that the лежащий в основе Fortran code is one of the major drawbacks. Since  ...

## Acknowledgements
Thanks @josiah for setting up an anime monorepo showing how to structure Rust-based multi-language bindings.
