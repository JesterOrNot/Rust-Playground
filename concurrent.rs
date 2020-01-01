fn main() {
    let time1 = std::time::Instant::now();
    let the_thread = std::thread::spawn(move || {
        for i in 1..5000 {
            println!("{}", i);
        }
    });
    for i in 5001..10000 {
        println!("{}", i);
    }
    // the_thread.join();
    let time2 = std::time::Instant::now().duration_since(time1);
    println!("{:?}", time2);
}
