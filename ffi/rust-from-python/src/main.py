from ctypes import cdll

lib = cdll.LoadLibrary("target/release/libfoo.so")

lib.hello_rust()
