use crate::command::append::Append;
use crate::command::{Command, list::List, read::Read, write::Write};

use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse_command(reader: &mut BufReader<&TcpStream>) -> Result<Command, String> {
    let mut line = String::new();

    // Read the first line: expected to be the command
    if let Err(err) = reader.read_line(&mut line) {
        let msg = format!("Failed to read from stream: {}\n", err);
        return Err(msg);
    }

    if line.eq_ignore_ascii_case("QUIT\n") || line.len() == 0 {
        return Ok(Command::Quit);
    } else if line.starts_with("LIST") {
        Ok(Command::List(List::new()))
    } else if let Some(rest) = line.strip_prefix("READ ") {
        let filename = rest.trim().to_string();
        Ok(Command::Read(Read::new(filename)))
    } else if let Some(rest) = line.strip_prefix("WRITE ") {
        let filename = rest.trim();
        let mut buffer = Vec::new();

        loop {
            let mut content_line = String::new();
            reader
                .read_line(&mut content_line)
                .map_err(|e| e.to_string())?;
            if content_line.trim_end() == "<EOF>" {
                break;
            }
            buffer.extend_from_slice(content_line.as_bytes());
        }

        Ok(Command::Write(Write::new(filename.to_string(), buffer)))
    } else if let Some(rest) = line.strip_prefix("APPEND ") {
        let filename = rest.trim();
        let mut buffer = Vec::new();

        loop {
            let mut content_line = String::new();
            reader
                .read_line(&mut content_line)
                .map_err(|e| e.to_string())?;
            if content_line.trim_end() == "<EOF>" {
                break;
            }
            buffer.extend_from_slice(content_line.as_bytes());
        }

        Ok(Command::Append(Append::new(filename.to_string(), buffer)))
    } else {
        Err(format!("Invalid command: {}", line))
    }
}
