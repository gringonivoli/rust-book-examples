use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello_server::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    let server = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in server.incoming() {
        pool.execute(|| {
            handle_connection(stream.unwrap());
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let in_buffer = BufReader::new(&mut stream);
    let request_line = in_buffer.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.xhtml"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.xhtml")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.xhtml"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
