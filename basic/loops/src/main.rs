fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut count = 0;

    'top_loop: loop {
        let mut inner = 10;

        loop {
            println!("inner: {}, count: {}", inner, count);
            if inner == 8 {
                break;
            }

            if count == 2 {
                break 'top_loop;
            }

            inner -= 1;
        }

        count += 1;
    }

    println!("end loop count: {}", count);

    let mut number = 3;

    while number != 0 {
        println!("current number is {}!", number);

        number -= 1;
    }

    let arr = [5; 5];

    for item in arr {
        println!("item: {}", item);
    }

    for item in (1..4).rev() {
        println!("item: {}", item);
    }
}
