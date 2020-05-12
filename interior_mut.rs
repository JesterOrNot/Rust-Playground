use std::cell::Cell;

#[derive(Debug)]
struct Point {
    x: i32,
    y: Cell<i32>
}

fn main() {
    let instance = Point { x: 5, y: Cell::new(3) };
    println!("{:?}", instance);
    instance.y.set(12);
    println!("{:?}", instance);
}
