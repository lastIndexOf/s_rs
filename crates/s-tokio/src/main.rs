use std::time::Duration;

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
}
