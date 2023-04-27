use std::io;
fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim().parse::<i32>().unwrap();

    println!("You entered: {}", input);

    if input < 10 {
        println!("Less than 10");
    } else if input >= 10 && input <= 20 {
        println!("Between 10 and 20");
    } else {
        println!("Greater than 20");
    }

    let input = if input > 20 {
        "more than 20"
    } else {
        "less than 20"
    };

    println!("{}", input);
}
