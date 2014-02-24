RUSTC ?= rustc
RUSTDOC ?= rustdoc

all: build sqlite lib bin

sqlite: build
	${RUSTC} --out-dir build/lib/ src/vendor/rustsqlite/src/sqlite3/lib.rs

lib: build sqlite
	${RUSTC} --out-dir build/lib -L build/lib src/docset.rs

bin: build lib
	${RUSTC} --out-dir build/bin -L build/lib docset.rs

build:
	mkdir -p build/lib
	mkdir -p build/bin

clean:
	rm -rf build

.PHONY: build sqlite lib bin
