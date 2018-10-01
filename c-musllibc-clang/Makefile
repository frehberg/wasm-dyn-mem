MUSL_LIBC_CLANG_HOME=/home/frehberg/tools/clang-wasm-toolchain/
SHELL=bash
TARGET=src/main.c
all:
	$(MUSL_LIBC_CLANG_HOME)/build/bin/wasm32-unknown-unknown-wasm-clang -Os $(TARGET) -o myapp.wasm
	wasm2wat myapp.wasm | grep -E "export|import"

clean:
	rm -f myapp.wasm 
