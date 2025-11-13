# Minimal Makefile for sonik

.PHONY: all build release install clean

all: release

build:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

clean:
	cargo clean
