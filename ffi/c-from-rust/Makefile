.PHONY: run clean

run: main clean
	./main
main: main.rs libfoo.a
	rustc main.rs -L .
libfoo.a: foo.o
	ar rcs libfoo.a foo.o
foo.o: foo.c
	clang-11 foo.c -c
clean:
	rm -rf *.out *.o *.a
