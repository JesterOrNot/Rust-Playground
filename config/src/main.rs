extern crate config;

use std::collections::HashMap;
use std::io;

fn main() -> io::Result<()> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();
    let data = settings.try_into::<HashMap<String, String>>().unwrap();
    let can_vote: &str = if data.get("age").unwrap().parse::<i32>().unwrap() >= 18 {
        "You can vote!"
    } else {
        "You can't vote!"
    };
    println!("Hello {}, {}", data.get("name").unwrap(), can_vote);
    Ok(())
}
