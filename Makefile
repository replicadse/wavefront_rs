.PHONY: update-version clean build release

update-version:
	sed 's/version = "0.0.0"/version = "$(VERSION)"/g' Cargo.toml > Cargo.toml.tmp
	mv Cargo.toml.tmp Cargo.toml

clean:
	cargo clean

build:
	cargo build

test:
	cargo test -- --nocapture

scan:
	cargo clippy --all-targets --all-features -- -D warnings

release:
	cargo build --release
