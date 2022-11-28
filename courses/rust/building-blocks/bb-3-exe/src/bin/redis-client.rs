use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    let ping = "*1\r\n$4\r\nPING/r/n";
    println!("PING");
    stream.write(&ping.as_bytes())?;
    let mut client_buffer = [0u8; 1024];
    stream.read(&mut client_buffer)?;

    let pong = str::from_utf8(&client_buffer).unwrap();
    println!("{}", pong);

    Ok(())
}