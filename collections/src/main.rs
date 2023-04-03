mod string;
mod vec;
mod hash_map;

use vec as vec_self;

fn main() {
    let arr = [5; 5];

    for (idx, &item) in arr.iter().enumerate() {
        println!("{}: {}", idx, item);
    }

    let mut vec = vec_self::create_vec_v1();
    let vec2 = vec_self::create_vec_v2(5, 5);

    vec.push(1);
    vec.push(2);
    vec.push(3);

    let three = &vec[2];

    println!("three is {three}");

    let five = if let Some(val) = vec2.get(4) {
        val
    } else {
        &-1
    };

    match vec2.get(4) {
        Some(val) => println!("in match five is {val}"),
        None => println!("in match five is -1"),
    }

    println!("five is {five}");

    string::create_str();
    hash_map::create_hash_map();
}
