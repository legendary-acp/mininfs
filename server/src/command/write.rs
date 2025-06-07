use std::{
    fs,
    io::{BufRead, BufReader, BufWriter},
    net::TcpStream,
};

use super::traits::Executable;
use crate::utils::{write_error, write_ok};

pub struct Write {
    pub filename: String,
}

impl Executable for Write {
    fn exec(
        &self,
        reader: &mut BufReader<&TcpStream>,
        writer: &mut BufWriter<&TcpStream>,
        base_dir: &std::path::Path,
    ) -> Result<Vec<u8>, String> {
        let complete_path = base_dir.join(&self.filename);
        let mut data = Vec::new();

        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    let msg = "Client closed connection while reading data".to_string();
                    write_error(writer, &msg);
                    return Err(msg);
                }
                Ok(_) if line.trim() == "<EOF>" => break,
                Ok(_) => data.extend_from_slice(line.as_bytes()),
                Err(e) => {
                    let msg = format!("Error reading data: {}", e);
                    write_error(writer, &msg);
                    return Err(msg);
                }
            }
        }

        if let Err(e) = fs::write(&complete_path, &data) {
            let msg = format!("Error while writing to file: {:#?}", e);
            write_error(writer, &msg);
            return Err(msg);
        }

        let msg = "WRITE executed successfully".as_bytes().to_vec();
        write_ok(writer, &msg);

        Ok(msg)
    }
}

impl Write {
    pub fn new(filename: String) -> Self {
        Self { filename }
    }
}
