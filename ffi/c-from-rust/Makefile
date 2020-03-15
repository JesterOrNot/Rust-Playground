.PHONY: run clean

run: main clean
	./main
main: src/main.rs libfoo.a
	rustc src/main.rs -L .
libfoo.a: foo.o
	ar rcs libfoo.a foo.o
foo.o: src/foo.c
	clang-11 src/foo.c -c
clean:
	rm -rf *.out *.o *.a
