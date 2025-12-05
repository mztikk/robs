set shell := ["nu", "--commands"]

test-all: build-all
    cargo hack nextest run --verbose --feature-powerset --workspace --no-tests pass

build-all:
    cargo hack build --verbose --no-dev-deps --feature-powerset --workspace

test-full $RUSTFLAGS="-D warnings":
    cargo fmt --all --check
    cargo machete
    cargo hack nextest run --feature-powerset --workspace --no-tests pass --stress-count=100
    cargo hack nextest run --feature-powerset --workspace --no-tests pass --release --stress-count=100

build:
    cargo build

test: build
    cargo nextest run

clean:
    cargo hack clean --verbose --workspace

run: build
    cargo run

bench:
    cargo bench
