pub mod append;
pub mod list;
pub mod read;
pub mod traits;
pub mod write;
use std::{io::BufWriter, net::TcpStream, path::Path};

use append::Append;
use list::List;
use read::Read;
use traits::Executable;
use write::Write;

pub enum Command {
    List(List),
    Read(Read),
    Write(Write),
    Append(Append),
}

impl Command {
    pub fn exec(
        &self,
        writer: &mut BufWriter<&TcpStream>,
        base_dir: &Path,
    ) -> Result<Vec<u8>, String> {
        match self {
            Command::List(cmd) => cmd.exec(writer, base_dir),
            Command::Read(cmd) => cmd.exec(writer, base_dir),
            Command::Write(cmd) => cmd.exec(writer, base_dir),
            Command::Append(cmd) => cmd.exec(writer, base_dir),
        }
    }
}
