use std::{fs::OpenOptions, io::Write};

use crate::utils::write_error;

use super::traits::Executable;
pub struct Append {
    pub filename: String,
    pub data: Vec<u8>,
}
impl Executable for Append {
    fn exec(
        &self,
        writer: &mut std::io::BufWriter<&std::net::TcpStream>,
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

        if let Err(e) = file.write_all(&self.data) {
            let msg = format!("Error while writing to file: {:#?}", e);
            write_error(writer, &msg);
            return Err(msg);
        }

        Ok("APPEND executed successfully".as_bytes().to_vec())
    }
}

impl Append {
    pub fn new(filename: String, data: Vec<u8>) -> Self {
        Append { filename, data }
    }
}
