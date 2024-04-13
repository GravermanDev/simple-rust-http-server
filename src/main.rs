use std::net::TcpListener;
use std::io::Write;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let result_ok = "HTTP/1.1 200 OK\r\n\r\n";

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write_all(result_ok.as_bytes()).unwrap();
                println!("Sent a 200 response");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
