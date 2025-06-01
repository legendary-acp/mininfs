use std::{
    error::Error,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub struct NFSClient {
    server_addr: String,
}

impl NFSClient {
    pub fn new(server_addr: &str) -> Self {
        NFSClient {
            server_addr: server_addr.to_string(),
        }
    }

    pub fn connect(self, command: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let mut stream = TcpStream::connect(self.server_addr)?;
        let mut reader = BufReader::new(stream.try_clone()?);

        // Send the command followed by newline
        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;

        let mut line = String::new();
        let mut content = Vec::new();

        // First line: OK or ERROR
        line.clear();
        let status = match reader.read_line(&mut line)? {
            0 => return Err("Server closed connection unexpectedly".into()),
            _ => line.trim().to_string(),
        };

        // Read rest until END
        loop {
            line.clear();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 || line.trim() == "END" {
                break;
            }
            content.push(line.trim().to_string());
        }

        match status.as_str() {
            "OK" => Ok(content),
            "ERROR" => Err(content.join("\n").into()),
            _ => Err(format!("Unknown status from server: {}", status).into()),
        }
    }
}
