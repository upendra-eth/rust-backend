use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener,TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("------------------starting server----------------------------");
    println!("1 - print ---- listener ------- {:?}", listener);

    for stream in listener.incoming() {
        // let stream = stream.unwrap();

        match stream {
            Ok(stream) => {
                // Handle the incoming connection in a separate thread or function.
                // Here, you would typically read from and write to the `stream`.
                println!("------------------ram ram from chrome----------------------------");

                println!("2 - print ---- stream ------------ {:?}", stream);
                println!("3 - print ---- TCP Connection established!");
                handle_connection(stream);
            }
            Err(e) => {
                // Handle errors, such as failed connections.
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    // stream.write_all(response.as_bytes()).unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
