mod cfg;
mod errors;

use cfg::Config;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    let args = vec![
        "/mnt/e/github/girios".to_string(),
        "--host".to_string(),
        "127.0.0.1:42069".to_string(),
    ];
    let cfg: Config = match Config::try_from(args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
        }
    };
    println!(r" ______ _________ ______  _________ _____    ______  ");
    println!(r"(  ___ |\__   __/(  __  ) \__   __/( ____ \ ( ______)");
    println!(r"| (        ) (   | (  ) |    ) (   | (   | || (      ");
    println!(r"| |        | |   | (__) |    | |   | |   | || (_____ ");
    println!(r"| |v0.1.1  | |   |     _)    | |   | |   | |(_____  )");
    println!(r"| | \_  )  | |   | (\ (      | |   | |   | )      ) |");
    println!(r"| (___) |__) (___| ) \ \_____) (___| (___| |/\____) |");
    println!(r"(_______)\_______/_)  \__/\_______/(_______)\_______) ");
    println!();
    println!("Connected to the server at {}", &cfg.host);
    println!(" ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("| Commands:                \t|");
    println!("|\tinsert <path> <value>   |");
    println!("|\tget <path>              |");
    println!("|\thit <path>              |");
    println!("|\tscan                    |");
    println!("|\tdelete <path>           |");
    println!(" ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");

    // Connect to the server
    let mut stream = TcpStream::connect(&cfg.host)?;
    let stdin = io::stdin();
    let mut reader = BufReader::new(stream.try_clone()?);
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;
        let input = input.trim_end();

        if input.eq_ignore_ascii_case("exit") {
            println!("Closing connection.");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Send input to the server
        stream.write_all(input.as_bytes())?;
        stream.flush()?; // Make sure data is sent immediately

        let mut response = String::new();

        // Read the response until the end delimiter ("\n\n")
        loop {
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => {
                    eprintln!("Server closed the connection.");
                    return Ok(()); // Graceful server shutdown
                }
                Ok(_) => {
                    response.push_str(&line);
                    if response.ends_with("\n\n") {
                        break; // Response is complete
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from server: {:?}", e);
                    return Err(e);
                }
            }
        }

        // Print the full response
        println!("{}", response.trim_end());
    }

    Ok(())
}
