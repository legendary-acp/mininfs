use std::io::{self, Write};

use client::NFSClient;

mod client;

fn main() {
    let addr = "127.0.0.1:8080";

    println!("Connected to miniNFS. Type commands (LIST, READ <file>, WRITE <file>)");
    loop {
        let client = NFSClient::new(addr);

        print!("> ");
        io::stdout().flush().unwrap(); // Make sure prompt appears

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }

        let command = input.trim();
        if command.is_empty() {
            continue;
        }

        match client.connect(command) {
            Ok(lines) => {
                for line in lines {
                    println!("{}", line);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
