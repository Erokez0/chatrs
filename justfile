# run the chat server
run mode:
    @if [ "{{ mode }}" = "prod" ]; then \
        cargo run --release; \
    else \
        RUST_LOG=DEBUG cargo run; \
    fi

# fix static code analysis warnings
lint:
    cargo clippy --fix
