pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines().filter(|line| line.contains(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "main";
        let text = "\
use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });

    if let Err(err) = run(&config) {
        process::exit(1);
    }
}";

        assert_eq!(vec!["fn main() {"], search(query, text));
    }
}
