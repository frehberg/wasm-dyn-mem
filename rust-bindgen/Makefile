SHELL=bash

all: build print-sizes

build:
	cd wasm-game-of-life-wee_alloc; wasm-pack build 
	cd wasm-game-of-life-wee_alloc-dyn; wasm-pack build
	cd wasm-game-of-life-dlmalloc-dyn; wasm-pack build
	cd wasm-game-of-life-dlmalloc; wasm-pack build

print-sizes: 
	@echo ">>>>>"	
	@echo ">>>>> printing wasm binary sizes"
	@ls -l wasm-game-of-life-wee_alloc/pkg/*.wasm  wasm-game-of-life-wee_alloc-dyn/pkg/*wasm wasm-game-of-life-dlmalloc-dyn/pkg/*.wasm wasm-game-of-life-dlmalloc/pkg/*.wasm

clean:
	cd wasm-game-of-life-wee_alloc; cargo clean; rm -rf bin
	cd wasm-game-of-life-wee_alloc-dyn; cargo clean; rm -rf bin
	cd wasm-game-of-life-dlmalloc-dyn; cargo clean; rm -rf bin
	cd wasm-game-of-life-dlmalloc; cargo clean; rm -rf bin


