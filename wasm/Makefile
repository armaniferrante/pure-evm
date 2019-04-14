#!/usr/bin/env gmake

.PHONY: all install serve

all: build install serve

build:
	cargo +nightly build --target=wasm32-unknown-unknown
	wasm-bindgen target/wasm32-unknown-unknown/debug/pure_evm_wasm.wasm --out-dir .

install:
	npm install

serve:
	npm run serve

clean:
	rm pure_evm_wasm*
	rm *~
