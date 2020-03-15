var ffi = require('ffi');

var lib = ffi.Library('target/release/libfoo', {
  'hello_rust': ['void', []]
});

lib.hello_rust();
