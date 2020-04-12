fn main() {
    fizzbuzz(100).iter().for_each(|n| println!("{}", n));
}

fn fizzbuzz(n: i32) -> Vec<String> {
    (1..=n).map(choose_val).collect()
}

fn choose_val(n: i32) -> String {
    if n % 15 == 0 {
        "FizzBuzz".to_owned()
    } else if n % 3 == 0 {
        "Fizz".to_owned()
    } else if n % 5 == 0 {
        "Buzz".to_owned()
    } else {
        n.to_string()
    }
}
