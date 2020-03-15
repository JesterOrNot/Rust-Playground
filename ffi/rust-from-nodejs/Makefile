.PHONY: all clean

all: clean target/release/libfoo.a
	node src/main.js
target/release/libfoo.a: src/lib.rs
	cargo build --release
clean:
	rm -rf target *.o *.a
