use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn test_mutual_exclusion() {
    let counter = Arc::new(Mutex::new(0));

    {
        let mut num = counter.lock().unwrap();

        *num += 1;
    }

    println!("Counter: {}", counter.lock().unwrap());

    let mut handles = vec![];
    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", counter.lock().unwrap());
}
