use std::thread;

fn main() {
    let list: Vec<i32> = (1..11).collect();

    thread::spawn(move || println!("list = {:?}", list));

    let numbers = [1, 2, 3, 4, 5];
    let product = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Product of all elements in array is {}", product);
}
