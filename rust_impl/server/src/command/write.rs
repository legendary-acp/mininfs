use std::fs;

use crate::utils::write_error;

use super::traits::Executable;
pub struct Write {
    pub filename: String,
    pub data: Vec<u8>,
}
impl Executable for Write {
    fn exec(
        &self,
        writer: &mut std::io::BufWriter<&std::net::TcpStream>,
        base_dir: &std::path::Path,
    ) -> Result<Vec<u8>, String> {
        let complete_path = base_dir.join(&self.filename);
        let _ = match fs::write(complete_path, &self.data) {
            Ok(c) => c,
            Err(e) => {
                let msg = format!("Error while writing to file: {:#?}", e);
                write_error(writer, &msg);
                return Err(msg);
            }
        };

        Ok("WRITE executed successfully".as_bytes().to_vec())
    }
}

impl Write {
    pub fn new(filename: String, data: Vec<u8>) -> Self {
        Write { filename, data }
    }
}
