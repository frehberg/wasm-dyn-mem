EMSDK_HOME=/home/frehberg/tools/emsdk
SHELL=bash
EM_ENV=/home/frehberg/tools/emsdk:/home/frehberg/tools/emsdk/clang/e1.38.12_64bit:/home/frehberg/tools/emsdk/node/8.9.1_64bit/bin:/home/frehberg/tools/emsdk/emscripten/1.38.12
TARGET=src/main.c

all:
	export PATH=$(EM_PATH):$(PATH); . $(EMSDK_HOME)/emsdk_env.sh; emcc  -Os $(TARGET) -o myapp.js
	wasm2wat myapp.wasm | grep -E "export|import" 

clean:
	rm -f myapp.wasm
