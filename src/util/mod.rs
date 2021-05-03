mod error;
pub use error::*;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

#[allow(dead_code)]
/// Given a vector of bytes, send a length and then the bytes
pub fn send_bytes(stream: &mut TcpStream, bytes_to_send: &Vec<u8>) -> TcpResult<()> {
    let len: u32 = bytes_to_send.len() as u32;
    // send the length, so we know how many bytes to receive
    stream
        .write(&len.to_ne_bytes())
        .map_err(|source| TcpError::SendError { source })?;
    // send the message
    stream
        .write(&bytes_to_send)
        .map_err(|source| TcpError::SendError { source })?;
    Ok(())
}

#[allow(dead_code)]
fn read_from_client(stream: &mut TcpStream, data: &mut [u8], data_len: usize) -> TcpResult<()> {
    let mut n: usize = 0;
    loop {
        let result = stream.read(&mut data[n..]);
        match result {
            Ok(0) => {
                println!("Client disconnected");
                return Err(TcpError::ClientDisconnectionError);
            }
            Ok(size) => {
                n += size;
                if n >= data_len {
                    return Ok(());
                }
            }
            Err(e) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                return Err(TcpError::IOError(e));
            }
        }
    }
}

#[allow(dead_code)]
pub fn read_bytes(mut stream: &mut TcpStream, mut bytes_to_read: &mut Vec<u8>) -> TcpResult<usize> {
    let mut data_len = vec![0u8; 4];
    read_from_client(stream, &mut data_len, 4)?;
    // no panic: u32::from_ne_bytes takes a 4 byte array, we know we have 4 bytes
    let data_len = u32::from_ne_bytes(data_len.try_into().unwrap()) as usize;
    println!("Receiving message of length:{}", data_len);
    read_from_client(&mut stream, &mut bytes_to_read, data_len)?;
    Ok(data_len)
}
