fn pow(m: i32, n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let mut x = pow(m, n.abs() / 2);

    if n % 2 == 0 {
        x = x * x;
    } else {
        x = x * x * m;
    }

    if n.is_positive() {
        return x;
    }

    1 / x
}

fn main() {
    assert_eq!(pow(2, 8), 256);
    assert_eq!(pow(2, -8), 1 / 256);
}
