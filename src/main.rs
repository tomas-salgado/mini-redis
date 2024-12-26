use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Read;
use std::collections::HashMap;

fn main() {
fn handle_client(mut stream: TcpStream) {
    println!("New client connected: {}", stream.peer_addr().unwrap());

    let mut buffer = [0; 1024];
    let mut data = HashMap::new();
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let request = String::from_utf8_lossy(&buffer[..n]).to_string();
                let parts: Vec<String> = request.split_whitespace().map(String::from).collect();

                match parts.get(0) {
                    Some(cmd) if cmd == "SET" && parts.len() >= 3 => {
                        let key = parts[1].clone();
                        let value = parts[2].clone();
                        data.insert(key, value);
                        stream.write(b"OK\n").unwrap();
                    }
                    Some (cmd) if cmd == "GET" && parts.len() >= 2 => {
                        let key = parts[1].clone();
                        let value = data.get(&key);
                        let response = match value {
                            Some(v) => format!("{}\n", v),
                            None => "nil\n".to_string(),
                        };
                        stream.write(response.as_bytes()).unwrap();
                    }
                    Some (cmd) if cmd == "DEL" && parts.len() >= 2 => {
                        let key = parts[1].clone();
                        let response = match data.remove(&key) {
                            Some(_) => "1\n",   // 1 key was removed
                            None => "0\n"       // no key existed
                        };
                        stream.write(response.as_bytes()).unwrap();
                    }
                    Some (cmd) if cmd == "EXISTS" && parts.len() >= 2 => {
                        let key = parts[1].clone();
                        let response = match data.contains_key(&key) {
                            true => "1\n",
                            false => "0\n",
                        };
                        stream.write(response.as_bytes()).unwrap();
                    }
                    _=> {
                        stream.write(b"ERROR: Unknown command\n").unwrap();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}

let listener = TcpListener::bind("0.0.0.0:6379").unwrap();

// accept connections and process them serially
for stream in listener.incoming() {
    handle_client(stream.unwrap());
}
}