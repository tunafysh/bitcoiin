use std::{io::Write, net::{TcpListener, TcpStream}};
use rand::Rng;
use sha2::{Sha256, Digest};
use owo_colors::OwoColorize;

mod structs;

fn handle_client(mut stream: TcpStream) {
    // Generate a valid data and target pair
    let server_block = generate_data();
    println!(
        "{}",
        format!("Sending data to client: {:?}", server_block.data).blue().bold()
    );

    // Serialize the ServerBlock and send it to the client
    let response = serde_json::to_string(&server_block).unwrap();
    if let Err(e) = stream.write(response.as_bytes()) {
        println!("{}", format!("Failed to send data to client: {}", e).red().bold());
    }
}

fn generate_data() -> structs::ServerBlock {
    let mut rng = rand::rng();
    let mut data = [0u8; 76]; // Example data size
    let nonce: u8 = rand::rng().random_range(15..=100);

    // Generate random data
    rng.fill(&mut data);

    // Concatenate data and nonce
    let mut concatenated_data = data.to_vec();
    concatenated_data.push(nonce);

    // Compute the double SHA-256 hash of the concatenated data
    let hash1 = Sha256::digest(&concatenated_data);
    let target = Sha256::digest(&hash1).to_vec();

    structs::ServerBlock {
        data: concatenated_data,
        target,
    }
}

fn main() {
    // Bind the server to port 7878
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("{}", "Server listening on port 7878".blue().bold());

    loop {
        // Accept incoming connections
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!(
                        "{}",
                        format!("New connection: {}", stream.peer_addr().unwrap())
                            .green()
                            .bold()
                    );

                    // Handle the client in the current thread
                    handle_client(stream);
                }
                Err(e) => {
                    println!("{}", format!("Connection failed: {}", e).red().bold());
                }
            }
        }
    }
}
