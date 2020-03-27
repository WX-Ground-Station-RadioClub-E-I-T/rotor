use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("ea4rct.org:4533")?;

    stream.write(format!("P {} {}", 70, 0).as_bytes())?;
    Ok(())
} // the stream is closed here