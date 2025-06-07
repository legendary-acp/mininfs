use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

use crate::utils::{write_error, write_ok};

use super::traits::Executable;
pub struct Append {
    pub filename: String,
}
impl Executable for Append {
    fn exec(
        &self,
        reader: &mut BufReader<&TcpStream>,
        writer: &mut BufWriter<&TcpStream>,
        base_dir: &std::path::Path,
    ) -> Result<Vec<u8>, String> {
        let complete_path = base_dir.join(&self.filename);
        let file_result = OpenOptions::new()
            .append(true)
            .create(true)
            .open(complete_path);

        let mut file = match file_result {
            Ok(f) => f,
            Err(e) => {
                let msg = format!("Error while opening file for append:{:#?}", e);
                write_error(writer, &msg);
                return Err(msg);
            }
        };

        let mut data: Vec<u8> = Vec::new();
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    let msg = "Client closed connection while reading data".to_string();
                    write_error(writer, &msg);
                    return Err(msg);
                }
                Ok(_) => {
                    if line.trim() == "<EOF>" {
                        break;
                    }
                    data.extend_from_slice(line.as_bytes());
                }
                Err(e) => {
                    let msg = format!("Error reading data: {}", e);
                    write_error(writer, &msg);
                    return Err(msg);
                }
            }
        }

        if let Err(e) = file.write_all(&data) {
            let msg = format!("Error while writing to file: {:#?}", e);
            write_error(writer, &msg);
            return Err(msg);
        }

        let msg = "APPEND executed successfully".as_bytes().to_vec();
        write_ok(writer, &msg);

        Ok(msg)
    }
}

impl Append {
    pub fn new(filename: String) -> Self {
        Append { filename }
    }
}
