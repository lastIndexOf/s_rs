use std::time::Duration;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = tokio::net::TcpListener::bind("0.0.0.0:8889").await?;

    loop {
        let (mut stream, _) = server.accept().await?;

        tokio::spawn(async move {
            let (reader, _) = stream.split();
            let read_buf = tokio::io::BufReader::new(reader);

            let mut lines = read_buf.lines();
            loop {
                let line = lines.next_line().await;

                println!("{line:?}");

                match line {
                    Ok(line) => {
                        if let Some(content) = line {
                            println!("{content}");

                            if content.is_empty() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }

            let res = "Hello".to_string();

            tokio::time::sleep(Duration::from_secs(5)).await;

            let _ = stream
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                        res.len(),
                        res
                    )
                    .as_bytes(),
                )
                .await;
            let _ = stream.flush().await;
        });
    }
}
