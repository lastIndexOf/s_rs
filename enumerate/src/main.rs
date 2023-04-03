#[derive(Debug)]
enum IpAddr {
    v4(u8, u8, u8, u8),
    v6(String),
}

#[derive(Debug)]
enum TestEnum {
    A,
    B { x: i32, y: i32 },
    C(i32, String),
    D(i32, i32, i32),
}

fn main() {
    let x = 5;
    let y = Some(5);
    let null: Option<i32> = None;

    let v4 = IpAddr::v4(127, 0, 0, 1);
    let v6 = IpAddr::v6(String::from("::1"));

    // dbg!(&TestEnum);
    dbg!(x);
    dbg!(&null);
    dbg!(&y);
    dbg!(&v4);
    dbg!(&v6);
}
