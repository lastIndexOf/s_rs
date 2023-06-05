use anyhow::Result;
use cute::c;
use std::time::Duration;
use tokio::{join, select, time::sleep, try_join};
use tokio_stream::StreamExt;

async fn read_book() -> i32 {
    println!("before read book");
    sleep(Duration::from_millis(1000)).await;
    println!("after read book");
    1
}

async fn listen_music() -> i32 {
    println!("before listen music");
    sleep(Duration::from_millis(1300)).await;
    println!("after listen music");
    2
}

async fn try_read_book() -> std::result::Result<i32, i32> {
    sleep(Duration::new(1, 0)).await;
    println!("after try read book");
    Ok(1)
}

async fn try_listen_music() -> std::result::Result<i32, i32> {
    sleep(Duration::new(1, 0)).await;
    println!("after try listen music");
    Err(32)
}

#[tokio::main]
async fn main() -> Result<()> {
    let arr = c![x, for x in 0..10, if x % 2 == 0];
    println!("arr created by c! = {arr:?}");
    sleep(Duration::new(1, 0)).await;
    println!("println after 2s");

    println!(
        "url = {}",
        "njttbw/dpn"
            .chars()
            .map(|it| std::char::from_u32(it as u32 - 1).unwrap())
            .collect::<String>()
    );

    join!(read_book(), listen_music());

    let a = try_join!(try_read_book(), try_listen_music());

    println!("a is error = {}", a.is_err());

    select! {
        _ = read_book() => {
            println!("read book has done in select");
        },
        _ = listen_music() => {
            println!("listen music has done in select");
        },
    };

    // wait select result
    sleep(Duration::from_millis(1000)).await;

    println!("==============================");

    Ok(())
}
