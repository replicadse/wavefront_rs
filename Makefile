.PHONY: init update-version clean build test scan release

init:
	rm -rf .git/hooks
	ln -s ../scripts/git-hooks .git/hooks
	chmod -R +x ./scripts/*

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
