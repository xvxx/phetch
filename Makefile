.PHONY: build release run

run: phetch
	./phetch

phetch: src/*.rs
	cargo build
	cp target/debug/phetch .

release:
	cargo build --release
	strip target/release/phetch

clean:
	rm -rf target
	fm -f phetch
