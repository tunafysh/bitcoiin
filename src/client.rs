use owo_colors::OwoColorize;
use dialoguer::{Input, theme::ColorfulTheme};
use std::{net::TcpStream, process::exit, io::{Read, Write}};
use sha2::{Digest, Sha256};

mod structs;

fn check_internet() -> bool {
    match reqwest::blocking::get("https://www.google.com/") {
        Ok(_) => true,
        Err(_) => false
    }
}

impl structs::ClientBlock {
    // Parse data received from the server into the ClientBlock
    fn parse_data(&mut self, data: structs::ServerBlock) {
        self.data = data.data;
        self.target = data.target;
        self.nonce = 0; // Reset nonce to 0 when new data is parsed
    }

    // Mine for a valid nonce
    fn mine(&mut self) {
        println!("{}", "Mining started...".blue().bold());

        loop {
            // Create a payload by combining the data and the current nonce
            let mut payload = self.data.clone();
            payload.extend_from_slice(&self.nonce.to_be_bytes());

            // Compute the double SHA-256 hash of the payload
            let hash1 = Sha256::digest(&payload);
            let hash2 = Sha256::digest(&hash1);

            // Compare the hash with the target
            if self.is_valid_hash(&hash2) {
                println!(
                    "{}",
                    format!("Valid nonce found: {}", self.nonce)
                        .green()
                        .bold()
                );
                break;
            }

            // Increment the nonce and continue mining
            self.nonce += 1;
        }
    }

    // Check if the hash is valid (i.e., less than or equal to the target)
    fn is_valid_hash(&self, hash: &[u8]) -> bool {
        hash <= self.target.as_slice()
    }

    // Serialize the ClientBlock into a JSON string
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

fn main() {
    println!("{}"," _     _ _            _ _       ");
    println!("{}","| |__ (_) |_ ___ ___ (_|_)_ __  ");
    println!("{}","| '_ \\| | __/ __/ _ \\| | | '_ \\ ");
    println!("{}","| |_) | | || (_| (_) | | | | | |");
    println!("{}","|_.__/|_|\\__\\___\\___/|_|_|_| |_|");

    println!("{}", "Client started".blue().bold());
    println!("{}", "Checking internet connection".blue().bold());
    if check_internet() {
        println!("{}", "Connected to the internet".green().bold());
    } else {
        println!("{}", "No internet connection".red().bold());
        exit(1);
    }

    let addr = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the server address (host:port)")
        .interact_text()
        .unwrap();

    let mut stream = match TcpStream::connect(&addr) {
        Ok(stream) => {
            println!("{}", "Connected to server".green().bold());
            stream
        }
        Err(_) => {
            println!("{}", "Failed to connect to server".red().bold());
            exit(1);
        }
    };

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let received_data = &buffer[..size];
            let server_block: structs::ServerBlock = serde_json::from_slice(received_data).unwrap();
            let mut client_block = structs::ClientBlock {
                data: Vec::new(),
                target: Vec::new(),
                nonce: 0,
            };
        
            // Parse data received from the server
            client_block.parse_data(server_block);
        
            // Start mining
            println!("{}", "Starting mining process".blue().bold());
            client_block.mine();
        
            // Send the result back to the server
            let result = client_block.to_json();
            stream.write_all(result.as_bytes()).unwrap();
            println!("{}", "Result sent to server".green().bold());        
        }
        Err(_) => {
            println!("{}", "Failed to receive data from server".red().bold());
            exit(1);
        }
    }
}
