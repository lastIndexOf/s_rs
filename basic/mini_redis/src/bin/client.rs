use std::time::Duration;

use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::{
    mpsc,
    oneshot::{self, Sender},
};

#[derive(Debug)]
pub enum Common {
    Get {
        key: String,
        sender: Responser<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        sender: Responser<()>,
    },
}

type Responser<T> = Sender<Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel::<Common>(32);

    tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379")
            .await
            .expect("Connect Redis Error");

        while let Some(data) = rx.recv().await {
            println!("Command = {:?}", data);
            match data {
                Common::Get { key, sender } => {
                    let result = client.get(&key).await;
                    let _ = sender.send(result);
                }
                Common::Set { key, value, sender } => {
                    let result = client.set(&key, value).await;
                    let _ = sender.send(result);
                }
            }
        }
    });

    let tx1 = tx.clone();
    tokio::spawn(async move {
        let (sender, tr) = oneshot::channel();

        let _ = tx1
            .send(Common::Set {
                key: String::from("Hello"),
                value: Bytes::from(String::from("World")),
                sender,
            })
            .await;

        let res = tr.await;
        println!("Get res = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (sender, tr) = oneshot::channel();

        if tx
            .send(Common::Get {
                key: String::from("Hello"),
                sender,
            })
            .await
            .is_err()
        {
            eprintln!("Error");
        };

        let res = tr.await;
        println!("Set res = {:?}", res);
        res
    });

    // t1.await.unwrap();
    println!("in out, result = {:?}", t2.await);

    // manager.await.unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
}
