.PHONY: build

run-wasmer4: build-wasmer4
	./target/debug/wasmer4

run-wasmer3: build-wasmer3
	./target/debug/wasmer3

build-wasmer3: build-wasm
	cargo build -p wasmer3

build-wasmer4: build-wasm
	cargo build -p wasmer4

build-wasm:
	cargo build -p wasm --target wasm32-wasi