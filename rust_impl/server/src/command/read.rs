use std::fs;

use crate::utils::{write_error, write_ok};

use super::traits::Executable;

pub struct Read {
    pub filename: String,
}

impl Executable for Read {
    fn exec(
        &self,
        writer: &mut std::io::BufWriter<&std::net::TcpStream>,
        base_dir: &std::path::Path,
    ) -> Result<Vec<u8>, String> {
        let complete_path = base_dir.join(&self.filename);

        println!("{:?}", complete_path);
        let _ = match fs::read(complete_path) {
            Ok(c) => {
                write_ok(writer, &c);
            }
            Err(e) => {
                let msg = format!("Error reading file: {:#?}", e);
                write_error(writer, &msg);
                return Err(msg);
            }
        };
        Ok("READ executed successfully".as_bytes().to_vec())
    }
}

impl Read {
    pub fn new(filename: String) -> Self {
        Read { filename }
    }
}
