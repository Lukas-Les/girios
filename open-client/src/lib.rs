use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn send_command(command: &str) -> io::Result<String> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:42069")?;
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

pub fn insert(path: &str, value: &str) -> io::Result<String> {
    let command = format!("insert {} {}", path, value);
    send_command(&command)
}

pub fn get(path: &str) -> io::Result<String> {
    let command = format!("get {}", path);
    send_command(&command)
}

pub fn hit(path: &str) -> io::Result<String> {
    let command = format!("hit {}", path);
    send_command(&command)
}

pub fn delete(path: &str) -> io::Result<String> {
    let command = format!("delete {}", path);
    send_command(&command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client() {
        let result = insert("wau", "audi");
        dbg!(result);
    }
}
