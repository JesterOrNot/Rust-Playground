#[link(name = "foo")]
extern {
    fn doubler(x: u8) -> u8;
    fn hello();
}

fn main() {
    unsafe {
        hello();
    }
}
