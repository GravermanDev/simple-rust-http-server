use std::net::{TcpStream, TcpListener};
use std::io::{BufRead, BufReader, Write};


fn send_path_in_body(mut stream: TcpStream, http_request: Vec<String>) {
    if let Some(request_line) = http_request.get(0) {
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "GET" {
            let path = parts[1];
            let path_parts: Vec<&str> = path.split("/").collect();
            if path_parts.len() == 2 && parts[0] == "echo" {
                let c = parts[1];
                let c_len = c.len();
                stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Lenght: {c_len}\r\n\r\n{c}\r\n\r\n").as_bytes()).unwrap();
            } else if parts[1] == "/"{
                send_ok(stream); // default path
            } else {
                send_not_found(stream); // invalid path
            }
        } else {
            send_not_found(stream); // no path
        }
    }
}

fn send_ok(mut stream: TcpStream) {
    stream.write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    println!("Send a Not Found response")
}

fn send_not_found(mut stream: TcpStream) {
    stream.write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
    println!("Send a Not Found response")
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    send_path_in_body(stream, http_request);
}

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
