# Makefile for sonik

FAKE_SCRIPT=./fake-device.sh

.PHONY: all build release install clean fake-on fake-off

all: release

build:
	cargo build

dev:
	RUST_LOG=debug cargo run

release:
	cargo build --release

install:
	cargo install --path .


uninstall:
	cargo uninstall || true
	sudo apt remove sonik || true

deb:
	sudo apt remove sonik || true
	cargo build --release
	cargo deb --no-build
	sudo dpkg -i target/debian/sonik_*.deb

clean:
	cargo clean

# --- Fake device commands -----------------------------------------

fake-on:
	@echo "[+] Attaching and auto-mounting fake device..."
	$(FAKE_SCRIPT) create

fake-off:
	@echo "[+] Unmounting and detaching fake device..."
	$(FAKE_SCRIPT) unmount
