fn multiplier(number: i32) -> impl Fn(i32)->i32 {
    move | multiplied | -> i32 {  number * multiplied }
}

fn main() {
    let times_three = multiplier(5);
    println!("{}", times_three(3))
}
