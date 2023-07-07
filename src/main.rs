use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use WebServer::{ThreadPool};

fn main() {
    const HOST: &str = "127.0.0.1:7878";
    let listener = TcpListener::bind(HOST).unwrap();
    let pool = ThreadPool::build(4).unwrap();

    dbg!("{}", &pool);
    dbg!(HOST);
    // Iterator over connections received
    for stream in listener.incoming() {
        let stream = stream.unwrap(); // unwrap the stream to get the TCP stream
        pool.execute(|| {
            handle_connection(stream);
        });

    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // read in the first line from the stream using .next on a BufReader
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // assign status_line and filename
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    // Empty body response
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
