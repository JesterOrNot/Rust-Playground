use std::net::TcpStream;
use std::io::{self, Read};

fn main() -> io::Result<()> {
  let mut client = TcpStream::connect("127.0.0.1:8080")?;
  client.set_nonblocking(true)?;
  loop {
    let mut buffer = [0;512];
    
    let incoming_text = match client.read(&mut buffer) {
      Ok(n) => n,
      Err(n) => 0,
    };
    println!("{}", String::from_utf8_lossy(&buffer[..]));
  }
}
