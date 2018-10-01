all:
	make -C c-emsdk
	make -C c-musllibc-clang
	make -C rust

clean:
	make -C c-emsdk clean
	make -C c-musllibc-clang clean
	make -C rust clean


