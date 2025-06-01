use std::{fs, net::TcpListener};

use clap::Parser;

mod command;
mod config;
mod handler;
mod utils;

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

fn start_server(config: config::config::Config) {
    let ip = "127.0.0.1";
    let address = format!("{}:{}", ip, config.port);

    println!("Starting server at {} on port {}", ip, config.port);

    let listener = TcpListener::bind(&address).expect("Failed to bind to address");
    println!("Server is running at {}", address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler::handler::handle_connection(stream, &config.path.clone())
    }
}

fn main() {
    let args = Args::parse();
    let config = config::config::Config::new(args.file_path, args.port);
    if let Err(e) = fs::create_dir_all(&config.path) {
        eprintln!(
            "Failed to create directory {:?}: {}",
            config.path.display(),
            e
        );
        std::process::exit(1);
    }
    start_server(config);
}
