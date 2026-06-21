# AI Usage Disclosure

This statement discloses the use of generative AI in the development of the
`sceua` software and its accompanying JOSS submission, in accordance with the
JOSS AI Usage Policy.

## Tools and models used

- OpenAI ChatGPT (GPT-5.5) accessed via API at planning stages of the project.
- Kimi K2.7-code were used for code writting and refactoring.

## Where AI was used

AI assistance was applied to the following components:

- **Rust core crate (`rust/src/`):** scaffolding and drafting of the SCE-UA
  algorithm translation from the original Fortran (Duan, 1992) and Matlab
  references, including `sce.rs`, `cce.rs`, `population.rs`, and `rng.rs`.
- **R bindings and `rextendr` integration (`r/`):** package scaffolding,
  `extendr` wrapper functions, `DESCRIPTION`, and CRAN packaging metadata.
- **Tests and benchmarks (`rust/tests/`, `rust/benches/`):** test scaffolding
  for the Duan test functions and Criterion benchmark setup.

AI was **not** used to draft the narrative text of the JOSS paper or the
project documentation (README files, Quarto site, vignettes), which were
written by the author.

## Nature and scope of assistance

AI assistance consisted of:

- Code generation and initial scaffolding of routines translated from
  Fortran/Matlab references.
- Refactoring and iterative editing of Rust code.
- Test and benchmark scaffolding.

All numerical behavior intended to mirror the original Fortran implementation
(RNG draw order, SCE-UA loop order, complex evolution sequence) was verified by
the author against the Fortran source and Duan's published test problems.

## Confirmation of human review

The author reviewed, edited, and validated every AI-assisted output before it
was committed. All core design decisions were made by the author, including:

- The decision to preserve Fortran-compatible serial behavior and RNG draw
  order for reproducibility against the reference implementation.
- The crate architecture and public API (`minimize`, `Config`).
- The choice of `rextendr` for R bindings and the monorepo layout.
- Algorithmic correctness checks against Duan's test problems.

The author remains solely responsible for the accuracy, originality,
licensing, and ethical/legal compliance of all submitted material.
