use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::thread::{self, spawn};
use std::time::Duration;

fn main() -> io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080")?;
  spawn(move || -> io::Result<()> {
    for stream in listener.incoming() {
      let stream = stream?;
      println!("Connection successful");
      handle_connection(stream)?;
    }
    Ok(())
  });
  loop {
    println!("Main Thread!");
    sleep();
  }
  Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
  let mut buffer = [0; 512];
  stream.read(&mut buffer)?;
  stream.write(b"pong")?;
  Ok(())
}

fn sleep() {
  thread::sleep(Duration::from_secs(5))
}