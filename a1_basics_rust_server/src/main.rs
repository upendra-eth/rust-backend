use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("------------------starting server----------------------------");
    println!("1 - print ---- listener ------- {:?}", listener);

    for stream in listener.incoming() {
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

// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let status_line = status_line;
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
