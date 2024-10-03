mod platform;
mod server;

use std::os::unix::process;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt}; 
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use server::requests::request_token::{RequestToken, process_token};


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let platform = Arc::new(RwLock::new(platform::Platform::new()));
    run_server(platform).await
}

async fn run_server(platform: Arc<RwLock<platform::Platform>>) -> tokio::io::Result<()> {
    const HOST: &str = "127.0.0.1";
    const PORT: &str = "42069";
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT)).await?;
    println!("Server listening on {}:{}", HOST, PORT);

    loop {
        let (socket, _) = match listener.accept().await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error accepting connection: {:?}", e);
                continue;
            }
        };

        match socket.readable().await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error reading from socket: {:?}", e);
                continue;
            }
        }

        let platform_ref = Arc::clone(&platform);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, platform_ref).await {
                eprintln!("Error handling connection: {:?}", e);
            }        
        });
    }
}

async fn handle_connection(mut socket: tokio::net::TcpStream, platform: Arc<RwLock<platform::Platform>>) -> tokio::io::Result<()> {
    let mut buffer = [0; 1024];
    loop {
        // Read data from the socket
        let n = match socket.read(&mut buffer).await {
            Ok(n) if n == 0 => return Ok(()), // Connection closed
            Ok(n) => n,
            Err(e) => return Err(e),
        };

        // Attempt to parse the request
        let request_token = match RequestToken::try_from(&buffer[..n]) {
            Ok(token) => token,
            Err(_) => {
                socket.write_all(b"Invalid request\n\n").await?;
                continue;
            }
        };
        match process_token(request_token, &platform).await {
            Ok(response) => {
                println!("Response: {}", &response);
                socket.write_all(b"Success\n").await?;
                socket.flush().await?;
                socket.write_all(response.as_bytes()).await?;
                socket.write_all(b"\n\n").await?;
                socket.flush().await?;
            }
            Err(e) => {
                socket.write_all(format!("Error processing request: {}\n\n", e).as_bytes()).await?;
            }
            
        }
    }
}