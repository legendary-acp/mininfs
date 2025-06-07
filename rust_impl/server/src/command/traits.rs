use std::io::{BufReader, BufWriter};
use std::net::TcpStream;
use std::path::Path;

pub trait Executable {
    fn exec(
        &self,
        reader: &mut BufReader<&TcpStream>,
        writer: &mut BufWriter<&TcpStream>,
        base_dir: &Path,
    ) -> Result<Vec<u8>, String>;
}
