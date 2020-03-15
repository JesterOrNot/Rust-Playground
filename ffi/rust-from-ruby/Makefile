.PHONY: all clean

all: clean target/release/libfoo.a
	ruby src/main.rb
target/release/libfoo.a: src/lib.rs
	cargo build --release
clean:
	rm -rf target *.o *.a
