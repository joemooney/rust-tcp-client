use anyhow::{Context, Result};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

use serde::{Deserialize, Serialize};
// use std::fs::File;
mod util;

#[derive(Serialize, Deserialize)]
struct Request {
    id: i8,
    payload: String,
}

fn main() -> Result<()> {
    let s = String::from("/tmp");
    //let b = s.into_bytes();

    println!("Sent command {}, awaiting reply...", s);

    println!("{}", s);
    let request = Request { id: 42, payload: s };

    let serialized_req = bincode::serialize(&request).context("Failed to serialize")?;

    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            util::send_bytes(&mut stream, &serialized_req).context("Failed to send bytes")?;

            let mut data = vec![0u8; 5000]; // using 1000 byte buffer
            let len = util::read_bytes(&mut stream, &mut data).context("Failed to read bytes")?;

            let text = from_utf8(&data[0..len]).context("Failed to convert UTF-8")?;
            println!("received: {}", text)
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
    Ok(())
}
