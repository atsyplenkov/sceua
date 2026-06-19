# Format Rust and R code
format:
    cargo fmt --all
    cd r && air format .

# Run Clippy on the Rust workspace
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

# Run Rust tests for the core SCE-UA crate
test-rs:
    cargo test -p sceua --locked
    cargo test -p sceua --all-features --locked

# Run Rust benchmarks for the core SCE-UA crate
rust-bench:
    cargo bench -p sceua

# Run jarl linter on the R package
jarl:
    cd r && jarl check .

# Build and test the R package
test-r:
    R CMD build r
    R CMD check --no-manual sceua_*.tar.gz
