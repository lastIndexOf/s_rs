use reqwest::header::{HeaderMap, HOST};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error, io::Write};

const HEADER_AUTH: &str = "Authorization";

#[derive(Serialize, Deserialize, Debug)]
struct ChatRequestBody {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: Role,
    content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
enum Role {
    system,
    user,
    assistant,
}

async fn ask_and_question(messages: Vec<Message>) -> Result<String, Box<dyn Error>> {
    let body = ChatRequestBody {
        model: "gpt-3.5-turbo-0613".to_string(),
        messages,
        stream: true,
    };
    // println!("request body = {}", serde_json::to_string_pretty(&body)?);

    let mut response = reqwest::Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header(
            HEADER_AUTH,
            "Bearer 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",
        )
        .json(&body)
        .send()
        .await?;

    while let Some(data) = response.chunk().await? {
        let s = String::from_utf8(data.to_vec())?;
        println!("in chunk data = {s}");
    }

    Ok("".to_string())

    // Ok(response["choices"][0]["message"]["content"].to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut messages = vec![Message {
        role: Role::system,
        content: "You are a helpful assistant.".to_string(),
    }];

    let mut writer = std::io::stdout();

    let _ = writer.write("Q: ".as_bytes());
    let _ = writer.flush();
    for input in std::io::stdin().lines() {
        match input {
            Ok(input) => {
                messages.push(Message {
                    role: Role::user,
                    content: input.clone(),
                });
                let response = ask_and_question(messages.clone()).await?;
                messages.push(Message {
                    role: Role::assistant,
                    content: response.clone(),
                });

                let _ = writer.write(format!("A: {response} \n").as_bytes());
                let _ = writer.flush();
                println!("");
            }
            Err(e) => {
                println!("err, {}", e);
            }
        }

        let _ = writer.write("Q: ".as_bytes());
        let _ = writer.flush();
    }

    Ok(())
}
