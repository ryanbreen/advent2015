.PHONY: all build run test

all: $(build)

build:
	@cargo build

run: $(build)
	cargo run

test:
	export RUST_MIN_STACK=33554432
	@cargo test
