use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("ram ram bhai .... address 127.0.0.1:8080 par bat suni ja rahi hae kripya wahi bole");

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("kuch aaaya hae ");

        tokio::spawn(async move {
            loop {
                match handle_socket(&mut socket).await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("failed to handle socket; err = {:?}", e);
                    }
                };
            }
        });
    }
}

async fn handle_socket(stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = [0; 1024];

    let n = match stream.read(&mut buffer).await {
        Ok(n) if n == 0 => return Ok(()), // socket closed
        Ok(n) => n,
        Err(e) => {
            eprintln!("failed to read from socket; err = {:?}", e);
            return Ok(());
        }
    };

    println!("aage aya hae");

    let res = String::from_utf8_lossy(&buffer[0..n]);

    let cookies = extract_cookies(&res);
    let user_id: &str = get_user_id(cookies);
    
    println!("we got user id - {}",user_id);

    // Custom message to send as the HTTP response body
    let response_message = "Hello, Client! This is a custom response message after showing you id";


    let response = format!("HTTP/1.1 200 OK\r\nX-User-Id: {}\r\nContent-Length: {}\r\n\r\n{}", user_id, response_message.len(),
    response_message);

    stream.write_all(response.as_bytes()).await?;
    stream.flush().await?;
    stream.shutdown().await?;

    Ok(())
}

fn get_user_id<'a>(mut cookies: impl Iterator<Item = &'a str>) -> &'a str {
    cookies
        .find(|cookie| cookie.starts_with("user_id"))
        .unwrap_or("")
        .split('=')
        .nth(1)
        .unwrap_or("")
}

fn extract_cookies(http: &str) -> impl Iterator<Item = &str> {
    let cookie_line = http
        .lines()
        .find(|line| line.starts_with("Cookie:"))
        .unwrap_or("");

    cookie_line
        .strip_prefix("Cookie:")
        .unwrap_or("")
        .split(';')
        .map(|cookie| cookie.trim())
}