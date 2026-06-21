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
      --locked \
      -- \
      -D warnings \
      -D clippy::dbg_macro

    cargo fmt

# Apply fixes reported by `just lint`
lint-rs-fix:
    cargo clippy \
      --all-targets \
      --locked \
      --fix --allow-dirty

    cargo fmt

# Run Rust tests for the core SCE-UA crate
test-rs:
    cargo test -p sceua --locked

# Run Rust benchmarks for the core SCE-UA crate
rust-bench:
    cargo bench -p sceua

# Run jarl linter on the R package
lint-r:
    cd r && jarl check .

# Build and test the R package.
# Bootstraps and vendors the Rust workspace into the R package source,
# then restores the original development Cargo.toml on exit.
test-r:
    #!/usr/bin/env bash
    set -euxo pipefail
    cp r/src/rust/Cargo.toml r/src/rust/Cargo.toml.bak
    cleanup() {
        mv r/src/rust/Cargo.toml.bak r/src/rust/Cargo.toml
        rm -rf r/src/rust/rust r/src/rust/vendor r/src/.cargo
    }
    trap cleanup EXIT
    cd r && Rscript bootstrap.R
    cd ..
    R CMD build r
    R CMD check --no-manual sceua_*.tar.gz
    rm -rf sceua_*.tar.gz sceua.Rcheck

# Build and install the R package locally.
install-r:
    #!/usr/bin/env bash
    set -euxo pipefail
    cp r/src/rust/Cargo.toml r/src/rust/Cargo.toml.bak
    cleanup() {
        mv r/src/rust/Cargo.toml.bak r/src/rust/Cargo.toml
        rm -rf r/src/rust/rust r/src/rust/vendor r/src/.cargo
        rm -f sceua_*.tar.gz
    }
    trap cleanup EXIT
    cd r && Rscript bootstrap.R
    cd ..
    R CMD build r
    R CMD INSTALL sceua_*.tar.gz

# Render the documentation website (syncs the Rust README first)
render:
    mkdir -p r/altdoc
    cp rust/README.md r/altdoc/rust_crate.md
    cd r && Rscript -e "altdoc::render_docs()"

# Preview the documentation website locally
preview:
    cd r && Rscript -e "altdoc::preview_docs()"

# Rust docs
docs:
    cargo doc -p sceua \
        --no-deps \
        --locked \
        --open
