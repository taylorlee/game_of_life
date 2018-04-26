
run: src/main.rs
	cargo web start --release --auto-reload

asm: src/main.rs
	cargo web start --target=asmjs-unknown-emscripten --release --auto-reload

deploy: src/main.rs
	cargo web deploy --release
