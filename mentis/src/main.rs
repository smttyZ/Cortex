use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};

fn main() {
    let listener =
        TcpListener::bind("0.0.0.0:3001").expect("Failed to bind port!");

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

    let filepath = if path == "/" {
        "../web-client/dist/index.html".to_string()
    } else {
        format!("../web-client/dist{}", path)
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