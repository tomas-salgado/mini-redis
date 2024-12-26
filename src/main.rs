use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
fn main() {
fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                println!("Received message: {}", String::from_utf8_lossy(&buffer[..n]));
                stream.write(&buffer[..n]).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

// accept connections and process them serially
for stream in listener.incoming() {
    handle_client(stream.unwrap());
}
}