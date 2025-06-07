use crate::command::append::Append;
use crate::command::{Command, list::List, read::Read, write::Write};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse_command(reader: &mut BufReader<&TcpStream>) -> Result<Command, String> {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .map_err(|err| format!("Failed to read from stream: {}\n", err))?;

    let command_line = line.trim();

    if command_line.eq_ignore_ascii_case("QUIT") || command_line.is_empty() {
        println!("Command Received: QUIT");
        return Ok(Command::Quit);
    } else if command_line.starts_with("LIST") {
        println!("Command Received: LIST");
        Ok(Command::List(List::new()))
    } else if let Some(rest) = command_line.strip_prefix("READ ") {
        println!("Command Received: READ");
        Ok(Command::Read(Read::new(rest.trim().to_string())))
    } else if let Some(rest) = command_line.strip_prefix("WRITE ") {
        println!("Command Received: WRITE");
        Ok(Command::Write(Write::new(rest.trim().to_string())))
    } else if let Some(rest) = command_line.strip_prefix("APPEND ") {
        println!("Command Received: APPEND");
        Ok(Command::Append(Append::new(rest.trim().to_string())))
    } else {
        Err(format!("Invalid command: {}", command_line))
    }
}
