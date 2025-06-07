use std::io::Write;

use client::NFSClient;

mod client;

fn main() {
    let server_addr = "127.0.0.1:8080";

    println!("Connecting to miniNFS server....");
    let mut client = NFSClient::new(server_addr).expect("Failed to connect to server");

    println!("Connected to miniNFS. Type commands (LIST, READ <file>, WRITE <file>, QUIT)");

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }
        let input = input.trim();
        if input.is_empty() {
            println!("Invalid Command");
            continue;
        }
        let mut cmd = String::new();
        if input.starts_with("WRITE ") {
            cmd.push_str(input);
            cmd.push('\n');
            println!("Enter file cotents (type <EOF> on a new line to finish):");
            loop {
                let mut data_line = String::new();
                std::io::stdin()
                    .read_line(&mut data_line)
                    .expect("Failed to read input");

                if data_line.trim() == "<EOF>" {
                    cmd.push_str("<EOF>\n");
                    break;
                } else {
                    cmd.push_str(&data_line);
                }
            }
        } else {
            cmd.push_str(input);
        }

        match client.send_command(&cmd) {
            Ok(lines) => {
                for line in lines {
                    println!("{}", line);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                if format!("{}", e).contains("Server closed connection") {
                    break;
                }
            }
        }
        if input.eq_ignore_ascii_case("QUIT") {
            println!("Disconnected.");
            break;
        }
    }
}
