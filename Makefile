.PHONY: build

run: build
	./target/debug/app

build: build-wasm
	cargo build

build-wasm:
	cargo build -p wasm --target wasm32-wasi