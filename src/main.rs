use std::collections::hash_map::HashMap;
use std::net::{TcpStream, TcpListener};
use std::io::{Write, Read};
use anyhow::Error;
use std::thread;

fn send_ok(stream: &mut TcpStream) {
    stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    println!("Sent a OK response")
}

fn send_not_found(stream: &mut TcpStream) {
    stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
    println!("Sent a Not Found response")
}

fn handle_echo(stream: &mut TcpStream, thing_to_echo: &str) {
    let len = thing_to_echo.len();
    let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\n\r\n{thing_to_echo}\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
    println!("Sent echo");
}

fn create_http_request_map(http_request: Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for line in http_request {
        let split_line: Vec<&str> = line.splitn(2, ": ").collect();
        if split_line.len() >= 2 {
            map.insert(split_line[0].to_string(), split_line[1].to_string());
        }
    }
    map
}

fn handle_user_agent(stream: &mut TcpStream, http_request: Vec<String>) {
    let map = create_http_request_map(http_request);

    if let Some(user_agent) = map.get("User-Agent") {
        let len = user_agent.len();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {len}\r\n\r\n{user_agent}\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        println!("Sent User-Agent");
    }
    send_not_found(stream);
}

fn handle_connection(stream: &mut TcpStream) -> Result<(), Error> {
    let mut buffer = [0u8; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let request_string = String::from_utf8_lossy(&buffer[..bytes_read]);
    let http_request: Vec<String> = request_string
        .lines()
        .map(|result| result.to_string())
        .take_while(|line| !line.is_empty())
        .collect();

    if let Some(request_line) = http_request.get(0) {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "GET" {
            let path = parts[1];
            let path_parts: Vec<&str> = path.splitn(3, "/").collect();

            if path == "/" {
                send_ok(stream);
                return Ok(());
            }

            match path_parts[1] {
                "echo" => handle_echo(stream, path_parts[2]),
                "user-agent" => handle_user_agent(stream, http_request),
                _ => send_not_found(stream),
            }
            return Ok(());
        }
    }

    send_not_found(stream);
    Ok(())
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                thread::spawn(move || match handle_connection(&mut stream) {
                    Ok(_) => (),
                    Err(error) => println!("Error handling connection: {}", error),
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

}
