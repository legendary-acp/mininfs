use std::{
    error::Error,
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
};

pub struct NFSClient {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl NFSClient {
    pub fn new(server_addr: &str) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(server_addr)?;
        let reader = BufReader::new(stream.try_clone()?);
        let writer = BufWriter::new(stream);
        Ok(NFSClient { reader, writer })
    }

    pub fn send_command(&mut self, cmd: &str) -> Result<Vec<String>, Box<dyn Error>> {
        self.writer.write_all(cmd.as_bytes())?;
        self.writer.write_all(b"\n")?;
        self.writer.flush()?;

        let mut line = String::new();
        let mut content = Vec::new();

        line.clear();
        let status = match self.reader.read_line(&mut line)? {
            0 => return Err("Server closed connection".into()),
            _ => line.trim().to_string(),
        };

        loop {
            line.clear();
            let bytes_read = self.reader.read_line(&mut line)?;
            if bytes_read == 0 || line.trim() == "END" {
                break;
            }
            content.push(line.trim().to_string());
        }
        match status.as_str() {
            "OK" => Ok(content),
            "ERROR" => Err(content.join("\n").into()),
            _ => Err(format!("Unkown status from server: {}", status).into()),
        }
    }
}
