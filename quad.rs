use std::io::{stdin, stdout,Write};

fn main() {
    loop {
        let mut a: String = String::new();
        let mut b: String = String::new();
        let mut c: String = String::new();
        print!("What is a?: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut a).unwrap();
        let a = match a.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a real number!");
                continue;
            },
        };
        print!("What is b?: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut b).unwrap();
        let b = match b.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a real number!");
                continue;
            },
        };
        print!("What is c?: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut c).unwrap();
        let c = match c.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please provide a real number!");
                continue;
            },
        };
        let quad = ((-b + f64::sqrt(b.powf(2.0)-4.0*a*c))/2.0*a, (-b - f64::sqrt(b.powf(2.0)-4.0*a*c))/2.0*a);
        println!("x = {}, and x = {}", quad.0, quad.1);
        match continue_or_no() {
            true => {
                println!("Ok..");
                continue;
            },
            false => {
                println!("Ok, Bye!!");
                return;
            }
        };
    }
}
fn continue_or_no() -> bool {
    let mut choice: String = String::new();
    print!("Do you want to continue?(y/n): ");
    stdout().flush().unwrap();
    stdin().read_line(&mut choice).unwrap();
    return if choice.trim() == "y" {
        true
    } else {
        false
    };
}