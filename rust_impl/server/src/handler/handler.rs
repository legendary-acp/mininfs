use std::{
    io::{BufReader, BufWriter},
    net::TcpStream,
    path::PathBuf,
};

use super::helper::parse_command;
use crate::utils::write_error;

pub fn handle_connection(stream: TcpStream, base_dir: &PathBuf) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    match parse_command(&mut reader) {
        Ok(cmd) => {
            if let Err(err) = cmd.exec(&mut writer, base_dir) {
                eprintln!("{}", err.trim_end());
            }
        }
        Err(err_msg) => {
            eprintln!("{}", err_msg.trim_end());
            write_error(&mut writer, &err_msg);
        }
    }
}
