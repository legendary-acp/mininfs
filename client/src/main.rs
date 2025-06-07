use std::io::Write;

use client::NFSClient;

mod client;

use clap::Parser;

/// miniNFS client
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Server address to connect to (e.g. 127.0.0.1:8080)
    #[arg(short, long, default_value = "127.0.0.1:8080")]
    server: String,
}

fn main() {
    let args = Args::parse();

    let server_addr = &args.server;
    println!("Connecting to miniNFS server @{}", server_addr);
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
        if input.starts_with("WRITE ") || input.starts_with("APPEND ") {
            cmd.push_str(input);
            cmd.push('\n');
            println!("Enter file cotents (type <EOF> on a new line to finish):");
            loop {
                let mut data_line = String::new();
                std::io::stdin()
                    .read_line(&mut data_line)
                    .expect("Failed to read input");

                if data_line.trim() == "<EOF>" {
                    cmd.push_str("<EOF>");
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
