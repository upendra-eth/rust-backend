use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("1 - print ---- listener ------- {:?}",listener);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("2 - print ---- stream ------------ {:?}",stream);

        println!("3 - print ---- TCP Connection established!");


        // match stream {
        //     Ok(stream) => {
        //         // Handle the incoming connection in a separate thread or function.
        //         // Here, you would typically read from and write to the `stream`.
        //     }
        //     Err(e) => {
        //         // Handle errors, such as failed connections.
        //         eprintln!("Error accepting connection: {}", e);
        //     }
        // }
    }
}