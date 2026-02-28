dev:
    RUST_LOG=DEBUG cargo run

lint:
    cargo clippy --fix
