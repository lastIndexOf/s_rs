fn main() {
    let i = 9;
    let x = match i {
        1 => "one",
        9 => "nine",
        _ => "other",
    };

    let five = Some(5u8);

    match five {
        Some(max) => println!("max is {}", max),
        _ => (),
    };

    let y = if let Some(val) = five { val + 10 } else { 0 };

    println!("x is {}", x);
    println!("y is {}", y);
}
