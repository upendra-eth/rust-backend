use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use a1_basics::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // create a thread pool

    println!("------------------starting server----------------------------");
    println!("1 - print ---- listener ------- {:?}", listener);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Handle the incoming connection in a separate thread or function.
                // Here, you would typically read from and write to the `stream`.
                println!(
                    "------------------ram ram from google crome ----------------------------"
                );

                println!("2 - print ---- stream ------------ {:?}", stream);
                println!("3 - print ---- TCP Connection established!");

                pool.execute(|| {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                // Handle errors, such as failed connections.
                println!("Error accepting connection: {}", e);
            }
        }
    }
}

// --snip--

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            println!("sleepeeeeeed");
            thread::sleep(Duration::from_secs(15));
            println!("awake");

            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let status_line = status_line;
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
