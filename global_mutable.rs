use std::sync::RwLock;

pub static mut count: RwLock<i32> = 0;

fn main() {
    unsafe {
        println!("{}", count);
        plus_five();
        println!("{}", count);
    }
}

fn plus_five() {
    unsafe { count += 5 }
}
