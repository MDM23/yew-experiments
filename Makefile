%:
	./.tpl/.deploy.sh "$@"

.PHONY: deps
deps:
	cargo install cargo-watch wasm-pack basic-http-server
