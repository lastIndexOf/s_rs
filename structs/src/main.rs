#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

struct Color(u8, u8, u8);
struct Point(u8, u8, u8);

struct Anywhere;

fn main() {
    let mut zfk = User {
        name: String::from("zhengfankai"),
        age: dbg!(26 + 1),
        email: String::from("kgaikj2cu@icloud.com"),
    };

    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    let _any = Anywhere;
    let _any2 = Anywhere;

    zfk.age += 1;

    print_name(&zfk);

    // let dmf = User {
    //     name: String::from("dengmeifang"),
    //     ..zfk
    // };

    println!("zfk's name is {}", zfk.name);
    println!("zfk's age is {}", zfk.age);
    println!("zfk's email is {}", zfk.email);
    println!("point.x = {}", origin.0);
    println!("white.r = {}", white.0);

    dbg!(&zfk);

    // println!("zfk = {}", zfk);
    println!("zfk = {:?}", zfk);
    println!("zfk = {:#?}", zfk);
}

fn print_name(user: &User) {
    println!("name {}", user.name);
}
