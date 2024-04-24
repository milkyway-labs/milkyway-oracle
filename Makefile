SHELL := /bin/bash

check:
	cargo check --target wasm32-unknown-unknown

build:
	cargo build

test:
	cargo test --locked --workspace

optimize:
	docker run --rm -v "$(CURDIR)":/code \
		--mount type=volume,source="$(notdir $(CURDIR))_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		--platform linux/amd64 \
		cosmwasm/rust-optimizer:0.14.0;
