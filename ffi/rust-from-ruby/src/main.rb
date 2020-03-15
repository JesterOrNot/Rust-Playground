require 'ffi'

module Hello
          extend FFI::Library
            ffi_lib 'target/release/libfoo.so'
              attach_function :hello_rust, [], :void
end

Hello.hello_rust