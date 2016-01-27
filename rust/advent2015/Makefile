.PHONY: all build run test

all: $(build)

build:
	@cargo build

run: $(build)
	cargo run -- -d $(day) -p $(part)

test:
	$(shell export RUST_MIN_STACK=33554432; cargo test)
