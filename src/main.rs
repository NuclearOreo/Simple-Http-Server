mod handlers;
mod process;
mod request;

use anyhow::Result;
use process::processing;
use std::net::TcpListener;
use std::thread;

const PORT: &str = "4221";
const ADDRESS: &str = "127.0.0.1";

fn main() -> Result<()> {
    let listener: TcpListener = TcpListener::bind(format!("{ADDRESS}:{PORT}"))?;
    println!("Listening to port {PORT}...");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                println!("accepted new connection");
                thread::spawn(move || match processing(s) {
                    Ok(_) => println!("Successfully closed stream"),
                    Err(e) => eprintln!("Failed to close stream: {:?}", e),
                });
            }
            Err(e) => eprintln!("error: {:?}", e),
        }
    }

    Ok(())
}
