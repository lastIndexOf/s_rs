fn main() {
    let x = {
        let x = 1;
        x + 1
    };

    println!("x = {}", callback(x));
}

fn callback(arg: i32) -> i32 {
    let x = {
        let x = 1;
        x + arg
    };

    x + 2
}
