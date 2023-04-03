use std::{error::Error, fs};

mod search;

pub struct Config {
    file_path: String,
    query: String,
}

impl Config {
    pub fn build<T>(mut args: T) -> Result<Config, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let query = if let Some(arg) = args.next() {
            arg
        } else {
            return Err("Didn't get a query string");
        };

        let file_path = if let Some(arg) = args.next() {
            arg
        } else {
            return Err("Didn't get a file path");
        };

        Ok(Config { query, file_path })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let text = fs::read_to_string(&config.file_path)?;

    for line in search::search(&config.query, &text) {
        println!("{line}");
    }

    Ok(())
}
