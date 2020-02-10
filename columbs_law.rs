fn columbs_law(k: i32, q1: i32, q2: i32, r: i32) {
    k*((q1*q2)/r.pow(2))
}
fn main() {
  println!("{}", columbs_law(3, 5, 4, 2))
}
