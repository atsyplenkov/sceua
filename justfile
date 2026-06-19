_default:
    just --list

# Format Rust and R code
format:
    cargo fmt --all
    cd r && air format .

# Run cargo clippy and cargo fmt
lint-rs:
    cargo clippy \
      --all-targets \
      --all-features \
      --locked \
      -- \
      -D warnings \
      -D clippy::dbg_macro

    cargo fmt

# Apply fixes reported by `just lint`
lint-rs-fix:
    cargo clippy \
      --all-targets \
      --all-features \
      --locked \
      --fix --allow-dirty

    cargo fmt

# Run Rust tests for the core SCE-UA crate
test-rs:
    cargo test -p sceua --locked
    cargo test -p sceua --all-features --locked

# Run Rust benchmarks for the core SCE-UA crate
rust-bench:
    cargo bench -p sceua

# Run jarl linter on the R package
lint-r:
    cd r && jarl check .

# Build and test the R package
test-r:
    R CMD build r
    R CMD check --no-manual sceua_*.tar.gz

# Render the documentation website (syncs the Rust README first)
render:
    mkdir -p r/altdoc
    cp rust/README.md r/altdoc/rust_crate.md
    cd r && Rscript -e "altdoc::render_docs()"

# Preview the documentation website locally
preview:
    cd r && Rscript -e "altdoc::preview_docs()"
