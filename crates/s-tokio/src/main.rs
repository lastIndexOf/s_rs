use std::time::Duration;

use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let _ = tokio::spawn(async {
        println!("in spawn");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("after 1s in spawn");
    });
    // .await;

    println!("out spawn");

    tokio::time::sleep(Duration::from_secs(2)).await;

    // Vec::<i32>::new().resize(new_len, value);
    println!(
        "{}",
        std::fs::read_dir("/Users/zhengfankai/workspace/metaforge/novel_spider/output")
            .unwrap()
            .count()
    );

    let (tx1, rx1) = tokio::sync::oneshot::channel();
    let (tx2, rx2) = tokio::sync::oneshot::channel();

    tokio::spawn(async {
        let _ = tx1.send("from one");
    });

    tokio::spawn(async {
        let _ = tx2.send("from one");
    });

    tokio::select! {
        val = rx1 => {
            println!("from rx1 = {val:?}");
        },
        val = rx2 => {
            println!("from rx1 = {val:?}");
        },
    };

    let mut data = vec![0_u8; 1024];

    tokio::select! {
        _ = async {
            let rc = std::rc::Rc::new(111);
            tokio::time::sleep(Duration::from_secs(1)).await;
            let data = data.clone();
            println!("rc = {rc:?} data = {data:?}");
        } => {
            data.push(1);
        },
        _ = async {
            let rc = std::rc::Rc::new(222);
            tokio::time::sleep(Duration::from_secs(1)).await;
            let data = data.clone();
            println!("rc = {rc:?} data = {data:?}");
        } => {
            data.push(2);
        },
        _ = async {
            let rc = std::rc::Rc::new(333);
            tokio::time::sleep(Duration::from_secs(1)).await;
            let data = data.clone();
            println!("rc = {rc:?} data = {data:?}");
        } => {
            data.push(3);
        },
        else => {}
    }

    let _ = tokio::spawn(async move {
        {
            let rc = std::rc::Rc::new(111);
            println!("rc = {rc:?}");
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
        let data = data.clone();
        println!("data = {data:?}");
        println!(" done ");
    })
    .await;

    let itr = tokio_stream::iter(vec![1, 2, 3, 4, 5]).take(3);

    tokio::pin!(itr);

    while let Some(val) = itr.next().await {
        println!("val = {val}");
    }

    loop {
        tokio::select! {
            val = tokio::signal::ctrl_c() => {
                println!("Exit with ctrl c: {val:?}");
                std::process::exit(1);
            },

            _ = tokio::time::sleep(Duration::from_millis(1000)) => {
                println!("Sleep for 1000ms");
            }
        }
    }
}
