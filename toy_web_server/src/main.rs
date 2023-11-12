use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // "We’ve chosen this port for two reasons: HTTP isn’t normally accepted on
    // this port so our server is unlikely to conflict with any other web server
    // you might have running on your machine, and 7878 is rust typed on a
    // telephone."
    
    // "The bind function in this scenario works like the new function in that
    // it will return a new TcpListener instance. The function is called bind
    // because, in networking, connecting to a port to listen to is known as
    // “binding to a port.”"

    // unwrap could technically cause a panic, but only if the wrapped type is
    // Err(E). In this case, we are confident that binding to loopback (127.0.0.1) on the
    // port 7878 will typically succeed, and return a Ok(T) in the option.

    // "Because we’re writing a basic server just for learning purposes, we
    // won’t worry about handling these kinds of errors; instead, we use unwrap
    // to stop the program if errors happen."

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "toy_web_server.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}