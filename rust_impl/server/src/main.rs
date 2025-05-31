use clap::Parser;
use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

#[derive(Parser, Debug)]
#[command(name = "miniNFS Server", about = "A minimal file server over TCP")]
struct Args {
    #[arg(
        short,
        long,
        value_name = "FILE_PATH",
        help = "Path to the file to be served"
    )]
    file_path: String,

    #[arg(
        short,
        long,
        default_value = "8080",
        help = "Port to bind the server on"
    )]
    port: u16,
}

struct Config {
    pub _root: String,
    pub port: u16,
}

fn handle_connection(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);

    let mut line = String::new();
    if let Ok(_) = reader.read_line(&mut line) {
        let command = line.trim();
        println!("Received Request for: {:?}", command)
    } else {
        eprintln!("Failed to read from stream");
        return;
    }
}

fn start_server(config: Config) {
    let ip = "127.0.0.1";
    let address = format!("{}:{}", ip, config.port);

    println!("Starting server at {} on port {}", ip, config.port);

    let listener = TcpListener::bind(&address).expect("Failed to bind to address");
    println!("Server is running at {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn main() {
    let args = Args::parse();
    let config = Config {
        _root: args.file_path,
        port: args.port,
    };
    start_server(config);
}
