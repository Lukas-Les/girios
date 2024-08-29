mod cfg;
mod errors;

use std::time::Duration;

use std::env;
use std::io::{self, Write, BufRead, BufReader};
use std::net::TcpStream;
use cfg::Config;

fn main() -> io::Result<()> {
    // let args: Vec<String> = env::args().collect();
    let args = vec!["/mnt/e/github/girios".to_string(), "--host".to_string(), "127.0.0.1:42069".to_string()];
    let cfg: Config = match Config::try_from(args) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
        }
    };

    // Connect to the server
    let mut stream = TcpStream::connect(&cfg.host)?;
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
    println!("Commands:");
    println!("\tinsert <path> <value>");
    println!("\tget <path>");
    println!("\thit <path>");
    println!("\tscan");
    println!("\tdelete <path>");


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

        stream.write_all(input.as_bytes())?;
        // stream.write_all(b"\n")?;
        stream.flush()?;

        // Set a short timeout for reading
        stream.set_read_timeout(Some(Duration::from_millis(100)))?;

        let mut response = String::new();
        loop {
            match reader.read_line(&mut response) {
                Ok(0) | Err(_) => break, // End of stream or timeout
                Ok(_) => {
                    print!("{}", response);
                    io::stdout().flush()?;
                    response.clear();
                }
            }
        }

        // Reset the timeout to None (blocking mode)
        stream.set_read_timeout(None)?;

        println!(); // Ensure we're on a new line before the next prompt
    }

    Ok(())
}
