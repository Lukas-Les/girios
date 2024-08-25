use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use common::dsa::char_tree::Tree;
use common::server::requests_handler::handle_request;


#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    // Create a shared, thread-safe instance of Tree
    let tree = Arc::new(Mutex::new(Tree::new()));

    // Start listening for incoming TCP connections
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on localhost:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;
        let tree = Arc::clone(&tree);

        // Spawn a new task to handle each connection
        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                // Read data from the socket
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return, // Connection closed
                    Ok(n) => n,
                    Err(_) => return,
                };

                let request = String::from_utf8_lossy(&buffer[..n]);
                let response = handle_request(&tree, &request).await;

                // Write the response back to the socket
                if let Err(_) = socket.write_all(response.as_bytes()).await {
                    return;
                }
            }
        });
    }
}