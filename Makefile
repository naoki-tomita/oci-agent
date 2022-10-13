build:
	cargo build --release

build-arm:
	cargo build --target aarch64-unknown-linux-gnu --release