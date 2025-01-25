use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main() {
    // Correct bind address
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // Create a thread pool with 4 threads

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read the HTTP request
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap(); // Read the first line of the request

    // Match the request line to determine the response
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hellow.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hellow.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Read the contents of the file
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| String::from("File not found"));
    let length = contents.len();

    // Construct the HTTP response
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Send the response to the client
    stream.write_all(response.as_bytes()).unwrap();
}
