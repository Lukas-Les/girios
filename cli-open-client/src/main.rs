mod cfg;
mod errors;

use std::env;
use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;

use cfg::Config;


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

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

        // Send the command to the server
        stream.write_all(command.as_bytes())?;
        stream.write_all(b"\n")?; // Optional: Send a newline after each command

        // Read the response from the server
        let mut response = String::new();
        reader.read_line(&mut response)?;

        // Display the server's response
        println!("Response: {}", response);
    }
    Ok(())
}
