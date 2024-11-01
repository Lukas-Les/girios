use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

struct Client {
    host: String,
}

impl Client {
    fn new(host: &str, port: usize) -> Self {
        Client {
            host: format!("{}:{}", host, port),
        }
    }

    fn send_command(&self, command: &str) -> io::Result<String> {
        // Connect to the server
        let mut stream = TcpStream::connect(&self.host)?;
        println!("Connected to the server.");

        // Send the command to the server
        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?; // Send a newline to indicate the end of the command

        // Read the response from the server
        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response)?;

        Ok(response)
    }

    pub fn insert(&self, path: &str, value: &str) -> io::Result<String> {
        let command = format!("insert {} {}", path, value);
        self.send_command(&command)
    }

    pub fn get(&self, path: &str) -> io::Result<String> {
        let command = format!("get {}", path);
        self.send_command(&command)
    }

    pub fn hit(&self, path: &str) -> io::Result<String> {
        let command = format!("hit {}", path);
        self.send_command(&command)
    }

    pub fn delete(&self, path: &str) -> io::Result<String> {
        let command = format!("delete {}", path);
        self.send_command(&command)
    }
}

