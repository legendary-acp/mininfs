use std::io::BufWriter;
use std::net::TcpStream;
use std::path::Path;

pub trait Executable {
    fn exec(&self, writer: &mut BufWriter<&TcpStream>, base_dir: &Path) -> Result<Vec<u8>, String>;
}
