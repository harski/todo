# Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
# Licensed under the 2-clause BSD license, see LICENSE for details.

PREFIX=/usr/local
TARGET=todo
DEBUG_BIN=target/debug/${TARGET}
RELEASE_BIN=target/release/${TARGET}

all: debug

install: release
	cp ${RELEASE_BIN} ${PREFIX}/bin/${TARGET}

${RELEASE_BIN}:
	cargo build --release

${DEBUG_BIN}:
	cargo build

release: ${RELEASE_BIN}

debug: ${DEBUG_BIN}

clean:
	rm -rf ${DEBUG_BIN} ${RELEASE_BIN}

run: ${DEBUG_BIN}
	cargo run

.PHONY: clean debug install release run
