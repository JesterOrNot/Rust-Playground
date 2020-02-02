fn main() {
    std::thread::spawn(move || {
        for i in 1..50 {
            print!("{} ", i);
        }
    });
    for i in 1..50 {
        print!("{} ", i);
    }
}
