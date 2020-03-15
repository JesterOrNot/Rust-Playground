.PHONY: all clean

all: clean target/release/libfoo.a
	python3 src/main.py
target/release/libfoo.a: src/lib.rs
	cargo build --release
clean:
	rm -rf target *.o *.a
