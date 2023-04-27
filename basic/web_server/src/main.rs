use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use thread_pool::ThreadPool;

fn main() {
    let server = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    for request in server.incoming().take(5) {
        let stream = request.unwrap();

        pool.execute(|| {
            handle_conn(stream);
        });
    }
}

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_status_line = buf_reader.lines().next().unwrap().unwrap();

    let (status, uri) = match &request_status_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/index.html"),
        _ => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 404 NOT FOUND", "static/404.html")
        }
    };

    let contents = fs::read_to_string(uri).unwrap();
    let length = contents.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
