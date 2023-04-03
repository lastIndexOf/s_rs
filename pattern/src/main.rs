struct Person {
    name: String,
    age: u8,
}

fn main() {
    let a = test_pattern();
    println!("a = {}", a);

    test_while();

    let b = (32, 32);
    test_tuple(b);

    print!("b = {:?}", b);

    let c = (32, String::from("32"));

    let (d, e) = c;

    println!("d = {}, e = {}", d, e);
    // println!("d = {:?}", c);

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    for i in 'a'..='z' {
        println!("{}", i);
    }

    let x = 5;

    match x {
        1..=6 => println!("one through six"),
        _ => println!("something else"),
    }

    let p = Person {
        name: String::from("zhengfankai"),
        age: 26,
    };

    let Person { name, age } = &p;

    println!("name = {}, age = {}", name, age);

    // println!("name = {:?}", p.name); // ownership error
    println!("name = {:?}", p.name); // ownership error
    println!("age = {:?}", p.age);

    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id @ 3..=7 } => println!("Found an id in range: {}", id),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

    let x = Some(10);

    match x {
        Some(y) if y > 5 && y < 15 => println!("y is between 5 and 15"),
        _ => println!("y is not between 5 and 15"),
    }
}

fn test_pattern() -> i32 {
    let c = true;
    let b = match c {
        true => return 5,
        false => 2,
    };

    b + 10
}

fn test_while() {
    let mut arr = vec![1, 2, 3];

    while let Some(x) = arr.pop() {
        println!("x = {}", x);
    }

    vec![5, 6, 7]
        .iter()
        .enumerate()
        .for_each(|(idx, item)| println!("idx = {}, item = {}", idx, item));
}

fn test_tuple((x, y): (i32, i32)) {
    println!("x = {}, y = {}", x, y);
}
