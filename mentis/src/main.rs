use std::{
    env,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};

fn main() {
    // Default values
    let default_host = "127.0.0.1";
    let default_port = "3001";
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Get host and port from arguments or use defaults
    let host = args.get(1).map_or(default_host, |s| s);
    let port = args.get(2).map_or(default_port, |s| s);
    
    let address = format!("{}:{}", host, port);
    
    println!("Starting server on {}", address);
    
    let listener = TcpListener::bind(&address)
        .expect(&format!("Failed to bind to {}!", address));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    if method != "GET" {
        send_response(&mut stream,
                      "HTTP/1.1 405 METHOD NOT ALLOWED",
                      "Method Not Allowed");
        return;
    }

    // Define the base path for static files
    let base_path = "../web-client/dist";

    let filepath = if path == "/" {
        format!("{}/index.html", base_path)
    } else {
        format!("{}{}", base_path, path)
    };

    if Path::new(&filepath).exists() {
        let contents =
            fs::read_to_string(&filepath).unwrap_or_else(|_| String::from("File read error"));
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        send_response(&mut stream, "HTTP/1.1 404 NOT FOUND", "404 Not Found");
    }
}

fn send_response(stream: &mut TcpStream, status_line: &str, message: &str) {
    let response = format!("{status_line}\r\nContent-Length: {}\r\n\r\n{}",
                           message.len(),
                           message);
    stream.write_all(response.as_bytes()).unwrap();
}