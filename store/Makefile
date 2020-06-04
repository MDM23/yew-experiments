RUST_TARGET ?= wasm32-unknown-unknown
RUST_PROFILE ?= debug
RUST_PACKAGE := $(shell basename $(CURDIR))

.PHONY: dev
dev:
	$(MAKE) -j2 serve build

.PHONY: serve
serve:
	basic-http-server ../target/web/$(RUST_PROFILE)/$(RUST_PACKAGE)

.PHONY: build
build:
	cargo watch -x 'build --target $(RUST_TARGET)' -s '$(MAKE) wasm'

.PHONY: wasm
wasm:
	wasm-bindgen \
		--target web \
		--no-typescript \
		--out-dir ../target/web/$(RUST_PROFILE)/$(RUST_PACKAGE) \
		--out-name $(RUST_PACKAGE) \
		../target/$(RUST_TARGET)/$(RUST_PROFILE)/$(RUST_PACKAGE).wasm
	cp -f index.html ../target/web/$(RUST_PROFILE)/$(RUST_PACKAGE)/
