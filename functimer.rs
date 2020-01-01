fn main() {
    timeFunction(&printer);
}
fn timeFunction(func: &dyn Fn()) {
    let time1 = std::time::Instant::now();
    func();
    let time2 = std::time::Instant::now().duration_since(time1);
    println!("{:?}", time2);
}
fn printer() {
    for i in 0..100 {
        println!("{}", i);
    }
}