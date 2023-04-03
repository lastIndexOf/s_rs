pub fn create_str() {
    let s1: &str = "hello world";
    let s2 = "hello ".to_string();
    let s3 = String::from("world");

    // let s4 = s2 + &s3;
    let s4 = format!("{s2}{s3}");

    println!("s1 = {s1}");
    println!("s2 = {s2}");
    println!("s3 = {s3}");
    println!("s4 = {s4}");

    let s5 = format!("{s3}, {s4}");

    println!("s3 = {s3}");
    println!("s4 = {s4}");
    println!("s5 = {s5}");

    let mut s6 = "hello ".to_string();

    s6.push_str("world");
    s6.push('!');

    println!("s6 = {s6}");
    println!("s6 length = {}", s6.len());

    // let s7 = format!("{s1}{s2}");

    let c1 = s6.chars().nth(11).unwrap();

    for &item in s6.as_bytes() {
        println!("item = {item}, {}", item == b'!');
    }

    println!("c1 = {c1}");
}
