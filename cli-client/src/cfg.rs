use std::path::PathBuf;

use crate::errors::ClientError;

pub struct Config {
    pub host: String,
    pub workdir: PathBuf,
}

impl TryFrom<Vec<String>> for Config {
    type Error = ClientError;

    fn try_from(mut args: Vec<String>) -> Result<Config, Self::Error> {
        let mut host: Option<String> = None;
        let workdir = args.remove(0).into();
        let args = args;
        // Iterate over the arguments
        let mut args_iter = args.into_iter();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "--host" => {
                    if let Some(value) = args_iter.next() {
                        host = Some(value);
                    } else {
                        return Err(ClientError::from_failed_config(
                            "Expected a value after '--host'".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(ClientError::from_failed_config(format!(
                        "Unknown argument: {}",
                        arg
                    )));
                }
            }
        }

        // Check that all required fields are provided
        let host = host.ok_or(ClientError::from_failed_config(
            "Missing required argument '--host'".to_string(),
        ))?;

        // Return the populated Config
        Ok(Config { host, workdir })
    }
}
