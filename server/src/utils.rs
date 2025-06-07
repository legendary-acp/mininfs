use std::io::{BufWriter, Write};
use std::net::TcpStream;

pub fn write_error(writer: &mut BufWriter<&TcpStream>, message: &str) {
    let full_msg = format!("ERROR\n{}\nEND\n", message);
    let _ = writer.write_all(full_msg.as_bytes());
    let _ = writer.flush();
}

pub fn write_ok(writer: &mut BufWriter<&TcpStream>, content: &Vec<u8>) {
    let _ = writer.write_all(b"OK\n");
    let _ = writer.write_all(content);
    let _ = writer.write_all(b"\nEND\n");
    let _ = writer.flush();
}
