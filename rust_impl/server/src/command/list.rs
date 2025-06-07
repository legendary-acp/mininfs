use std::{
    fs,
    io::{BufReader, BufWriter},
    net::TcpStream,
};

use crate::utils::{write_error, write_ok};

use super::traits::Executable;

pub struct List;

impl Executable for List {
    fn exec(
        &self,
        _reader: &mut BufReader<&TcpStream>,
        writer: &mut BufWriter<&TcpStream>,
        base_dir: &std::path::Path,
    ) -> Result<Vec<u8>, String> {
        let entries = match fs::read_dir(base_dir) {
            Ok(e) => e,
            Err(e) => {
                let msg = format!("Failed to read directory: {:#?}", e);
                write_error(writer, &msg.as_str());
                return Err(msg);
            }
        };

        let mut output = String::new();

        let _res = for entry in entries {
            match entry {
                Ok(e) => {
                    let filename = e.file_name().into_string().unwrap_or_default();
                    if filename.starts_with('.') {
                        continue;
                    }
                    output.push_str(&filename);
                    output.push_str(" ");
                }
                Err(e) => {
                    let msg = format!("Failed to read directory entry: {}", e);
                    write_error(writer, &msg);
                    return Err(msg);
                }
            }
        };

        write_ok(writer, &output.into_bytes());

        Ok("LIST executed successfully".as_bytes().to_vec())
    }
}

impl List {
    pub fn new() -> Self {
        Self
    }
}
