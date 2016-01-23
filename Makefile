# Copyright 2016 Tuomo Hartikainen <tth@harski.org>.
# Licensed under the 2-clause BSD license, see LICENSE for details.

PREFIX=/usr/local
TARGET=todo
#FLAGS="--release"

all: debug

install: release
	cp $< ${PREFIX}/${TARGET}

target/release/${TARGET}:
	cargo build --release

target/debug/${TARGET}:
	cargo build

release: target/release/${TARGET}

debug: target/debug/${TARGET}

clean:
	rm -rf target/debug/${TARGET} target/release/${TARGET}

run: target/debug/${TARGET}
	cargo run

.PHONY: clean debug install release run
