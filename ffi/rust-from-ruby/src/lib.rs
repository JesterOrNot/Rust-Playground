#![crate_type = "staticlib"]

#[no_mangle]
pub extern fn hello_rust() {
    println!("Hello World From Rust");
}
