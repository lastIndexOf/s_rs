use std::time::Duration;

use anyhow::Result;
use cute::c;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    let arr = c![x, for x in 0..10, if x % 2 == 0];
    println!("arr created by c! = {arr:?}");
    sleep(Duration::new(1, 0)).await;
    println!("println after 2s");

    // let a = || {
    //     let c = arr;
    //     println!("c = {c:?}");
    // };

    // Error:
    // let b = || {
    //     let c = arr;
    //     println!("c = {c:?}");
    // };

    // async {
    //     let arr = arr;
    //     println!("arr = {arr:?}");
    // }
    // .await;

    // async {
    //     println!("arr = {arr:?}");
    // }
    // .await;

    println!(
        "url = {}",
        "njttbw/dpn"
            .chars()
            .map(|it| std::char::from_u32(it as u32 - 1).unwrap())
            .collect::<String>()
    );

    Ok(())
}
