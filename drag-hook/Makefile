RUST_TARGET ?= wasm32-unknown-unknown
RUST_PROFILE ?= debug
RUST_PACKAGE := $(shell basename $(CURDIR))

.PHONY: dev
dev:
	$(MAKE) -j2 watch serve

.PHONY: watch
watch:
	cargo watch -i "pkg/*" -s "make build"

.PHONY: build
build:
	wasm-pack build --debug --no-typescript --target=web --out-name=$(RUST_PACKAGE)

.PHONY: serve
serve:
	basic-http-server .
