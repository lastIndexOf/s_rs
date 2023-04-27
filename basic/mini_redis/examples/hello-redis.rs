use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("Hello", "World".into()).await?;

    let result = String::from_utf8(client.get("Hello").await?.unwrap().to_vec()).unwrap();

    println!("result = {:?}", result);

    Ok(())
}
