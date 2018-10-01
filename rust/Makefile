SHELL=bash

all:
	cd myapp; cargo +nightly build --target wasm32-unknown-unknown --release 
	wasm2wat myapp/target/wasm32-unknown-unknown/release/myapp.wasm | grep -E "export|import"
	ls -l myapp/target/wasm32-unknown-unknown/release/myapp.wasm
	wasm2wat -v myapp/target/wasm32-unknown-unknown/release/myapp.wasm | grep debug

clean:
	cd myapp; cargo clean

