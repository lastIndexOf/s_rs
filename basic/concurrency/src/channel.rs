use std::{sync::mpsc, thread, time::Duration};

pub fn test_channel() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let hake_hands = String::from("hello world");

        thread::sleep(Duration::from_secs(1));
        match tx.send(hake_hands) {
            Ok(_) => println!("Sent message"),
            Err(_) => println!("Failed to send message"),
        };

        // println!("Sent message {}!", hake_hands);
    });

    println!("Waiting for message...");

    match rx.recv() {
        Ok(msg) => println!("Got message: {}", msg),
        Err(_) => println!("Failed to receive message"),
    }

    println!("Receive message!");

    match handle.join() {
        Ok(_) => println!("Thread finished"),
        Err(_) => println!("Thread panicked"),
    }
}

pub fn test_channel_multiple() {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let msgs = vec!["hello", "from", "the", "other", "side"];

        for msg in msgs {
            match tx.send(msg) {
                Ok(_) => println!("Sent message: {msg}"),
                Err(_) => println!("Failed to send message"),
            };
        }
    });

    for msg in rx {
        println!("Got message: {}", msg);
    }

    match handle.join() {
        Ok(_) => println!("Thread finished"),
        Err(_) => println!("Thread panicked"),
    }
}

pub fn test_multiple_producer_signal_consumer() {
    let (tx, rx) = mpsc::channel();

    let child_tx_1 = tx.clone();

    let handle_1 = thread::spawn(move || {
        let messages = vec!["from the first thread", "first message"];

        for msg in messages {
            match child_tx_1.send(msg) {
                Ok(_) => (),
                Err(_) => println!("Failed to send message"),
            };
            thread::sleep(Duration::from_secs(1));
        }
    });
    let handle_2 = thread::spawn(move || {
        let messages = vec!["from the second thread", "second message"];

        for msg in messages {
            match tx.send(msg) {
                Ok(_) => (),
                Err(_) => println!("Failed to send message"),
            };
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("Got message: {}", msg);
    }

    println!("Got message Done");

    match handle_1.join() {
        Ok(_) => println!("Thread 1 finished"),
        Err(_) => println!("Thread 1 panicked"),
    }

    match handle_2.join() {
        Ok(_) => println!("Thread 2 finished"),
        Err(_) => println!("Thread 2 panicked"),
    }
}
