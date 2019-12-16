.PHONY: build release run

run:
	cargo run

build:
	cargo build

release:
	cargo build --release
	strip target/release/phetch

clean:
	rm -rf target
