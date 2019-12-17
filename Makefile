.PHONY: build release run dev debug

dev: debug

debug: target/debug/phetch
release: target/release/phetch

target/debug/phetch: src/*.rs
	cargo build
	cp $@ phetch

target/release/phetch: src/*.rs
	cargo build --release
	strip $@
	cp $@ phetch

clean:
	rm -rf target
	rm -f phetch
