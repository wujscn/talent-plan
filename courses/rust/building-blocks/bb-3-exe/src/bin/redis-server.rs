
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process;

fn handle_client(mut stream: TcpStream)-> std::io::Result<()> {
    let mut buf = [0u8; 1024];
    while let Ok(_len) = stream.read(&mut buf) {
        stream.write("+PONG".as_bytes())?;
        process::exit(0);
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("Redis fake version, listening to 127.0.0.1:6379...");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}