
default: debug

debug:
    cargo run

devsetup:
    cp dev/hooks/* .git/hooks

check:
    cargo check --workspace

fmt:
    cargo +nightly fmt --all

lint:
    cargo clippy -- -W clippy::unwrap_used -W clippy::cargo
