use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};
use web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();

        // println!("Connecting established!");

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_name) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "front/index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "front/index.html")
        }
        _ => ("HTTP/1.1 303 See Other", "front/error.html"),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // println!("{response}");

    stream.write_all(response.as_bytes()).unwrap();
}
