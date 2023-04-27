fn main() {
    let s1 = "hello world";
    let mut s2 = String::from("hello world");

    let first = first_word(&s2);
    println!("{}", first_word(s1));
    println!("{}", first);

    println!("{}", first);

    s2.clear();
}

fn first_word(string: &str) -> &str {
    for (i, &item) in string.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &string[0..i];
        }
    }

    &string[..]
}
