use std::{thread, time};

pub fn basic() {
    let handle = thread::spawn(|| {
        for idx in 1..15 {
            println!("Hello from a child thread! {idx}");
            thread::sleep(time::Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for idx in 1..5 {
        println!("Hello from a main thread! {idx}");
        thread::sleep(time::Duration::from_millis(2));
    }
}

pub fn move_thread() {
    let list = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:#?}", list);
    });

    handle.join().unwrap();
}
