
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

use bb_3_exe::RedisCmd;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;

    let ping = RedisCmd::Array{
        data: vec!["*1".to_string(), "$4".to_string(), "PING".to_string()],
    };
    let ping = serde_json::to_string(&ping).unwrap();
    println!("client> PING");
    stream.write(&ping.as_bytes())?;
    let mut client_buffer = [0u8; 256];
    let len = stream.read(&mut client_buffer)?;

    let received = &str::from_utf8(&client_buffer).unwrap()[0..len];
    let received: RedisCmd = serde_json::from_str(received)?;

    println!("client> {:?}", received);

    Ok(())
}