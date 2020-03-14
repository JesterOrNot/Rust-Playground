.PHONY: all clean

all: main clean
	./main
main: main.o libfoo.a
	gcc -o main main.o libfoo.a  -Wl,--gc-sections -lpthread -ldl
libfoo.a: src/lib.rs Cargo.toml
	cargo build --release && \
	mv target/release/libfoo.a libfoo.a
main.o: src/main.c
	gcc -o main.o -c src/main.c
clean:
	rm -rf target *.o *.a
