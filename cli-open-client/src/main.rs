mod cfg;
mod errors;

use std::env;
use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;

use cfg::Config;


fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    let args = vec!["/mnt/e/github/girios".to_string(), "--host".to_string(), "127.0.0.1:42069".to_string()];

    let cfg: Config = match Config::try_from(args) {
        Ok(config) => config,  // If successful, store the Config in cfg
        Err(e) => {
            eprintln!("Error: {}", e);  // Print the error message
            return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
        }
    };

    // Connect to the server
    let mut stream = TcpStream::connect(&cfg.host)?;
    println!(r"_______ _________ _______ _________ _______  _______ ");
    println!(r"(  ____ \\__   __/(  ____ )\__   __/(  ___  )(  ____ \");
    println!(r"| (    \/   ) (   | (    )|   ) (   | (   ) || (    \/");
    println!(r"| |         | |   | (____)|   | |   | |   | || (_____ ");
    println!(r"| | ____    | |   |     __)   | |   | |   | |(_____  )");
    println!(r"| | \_  )   | |   | (\ (      | |   | |   | |      ) |");
    println!(r"| (___) |___) (___| ) \ \_____) (___| (___) |/\____) |");
    println!(r"(_______)\_______/|/   \__/\_______/(_______)\_______)");
    println!();
    
    println!("Connected to the server at {}", &cfg.host);

    let mut input = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        // Prompt the user for input
        print!("> ");
        io::stdout().flush()?; // Ensure the prompt is printed

        // Read user input from stdin
        input.clear();
        stdin.read_line(&mut input)?;

        // Trim the input to remove any newline or whitespace
        let command = input.trim();

        // If the user types "exit", break the loop and close the connection
        if command.eq_ignore_ascii_case("exit") {
            println!("Closing connection.");
            break;
        }
        if command.is_empty() {
            continue; // Go back to the start of the loop without sending to the server
        }

        // Send the command to the server
        stream.write_all(command.as_bytes())?;
        // stream.write_all(b"\n")?; // Send a newline after each command to signal end of input

        // Read all available responses from the server
        loop {
            let mut response = String::new();
            let bytes_read = reader.read_line(&mut response)?;

            if bytes_read == 0 {
                // No more data to read; break out of the loop
                break;
            }

            // Display the server's response
            println!("{}", response);

            // Optionally, add logic here to determine if you've received all responses
            // For example, if you know the server sends a specific end marker, check for that
            // If the response indicates the end of a transmission or is empty, break
            if response.trim().is_empty() || response.trim() == "END" {
                break;
            }
        }
    }

    // Explicitly drop the stream to close the connection
    drop(stream);
    Ok(())
}
