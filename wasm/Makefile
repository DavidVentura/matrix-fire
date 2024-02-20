.PHONY: run
run:
	wasm-pack build --target web && python3 -m http.server
test:
	cargo test --lib --target x86_64-unknown-linux-gnu
