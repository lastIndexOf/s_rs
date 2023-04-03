fn main() {
    let add = 1 + 2;
    let sub = 1 - 2;
    let mul = 1 * 2;
    let div = 5 / 2;
    let div_f = 5.0 / 2.0;
    let rem = 7 % 3;
    let _a: u8 = b'A';

    let bol = true;
    let heart_eyed_cat = '😻';

    let tup = (65, 6.4, b'A');

    let _unit = ();

    let arr: [i32; 5] = [1; 5];

    println!(
        "{} {} {} {} {} {} {} {} {} {} {} {} {}",
        add, sub, mul, div, div_f, rem, _a, bol, heart_eyed_cat, tup.2, arr[0], arr[1], arr[2]
    );

    let x = 10;

    test_pointer(x);
}

fn test_pointer(x: i32) {
    let b = &x;

    let c: i32 = *b;

    println!("x = {}, {}", b, c);
}
