use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let v = vec![1, 2, 3];

    v[99];

    Ok(())
}

fn read_file_text(file_name: &str) -> Result<String, io::Error> {
    let mut res = String::new();
    File::open(file_name)?.read_to_string(&mut res)?;

    Ok(res)
}

// fn test() -> Result<char, Box<dyn Error>> {
//     let s = String::from("hello world");

//     s.chars().nth(19).ok_or()?;

//     Ok('Y')
// }
