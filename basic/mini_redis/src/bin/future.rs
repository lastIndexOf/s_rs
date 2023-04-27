use std::{rc::Rc, sync::Arc, thread, time::Duration};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    // delay().await;

    test_select().await;

    match tokio::signal::ctrl_c().await {
        Ok(_) => println!("exit with ctrl c, {}", stringify!(Hello, World)),
        _ => (),
    }
}

async fn delay() {
    let notify = Arc::new(tokio::sync::Notify::new());

    let notify_cloned = Arc::clone(&notify);

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));

        notify_cloned.notify_one();

        thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));

            notify_cloned.notify_one();
        });
    });

    notify.notified().await;

    println!("after 2s in delay");

    notify.notified().await;

    println!("after 2s in delay");
}

async fn test_select() {
    let (mut tx1, mut rx1) = tokio::sync::mpsc::channel::<i32>(128);
    let (mut tx2, mut rx2) = tokio::sync::mpsc::channel::<i32>(128);

    let ss = "he".to_string();
    tokio::spawn(async move {
        // 用 tx1 和 tx2 干一些不为人知的事
        tokio::time::sleep(Duration::from_secs(1)).await;
        drop(tx1);

        {
            let bc = Rc::new(2);
        }
        // let bc = Rc::new(2); // Error

        tokio::time::sleep(Duration::from_secs(1)).await;
        // drop(tx2);
        let _ = tx2.send(32).await;
    });

    tokio::select! {
        rx1_val = async {
            let a = Rc::new(222);
            rx1.recv().await
        } => {
            println!("from rx1 = {:?}", rx1_val);
        }
        Some(rx2_val) = rx2.recv() => {
            println!("from rx2 = {}", rx2_val);
        }
        else => {
            println!("Some Error");
        }
    }

    println!("select End");

    let mut stream = tokio_stream::iter([1, 2, 3]);

    while let Some(item) = stream.next().await {
        println!("item = {item}");
    }

    let mut v = vec![1, 2, 3, 4, 5];
    v.swap_remove(0);

    println!("{:?}", v);
}
