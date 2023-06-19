use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct AIProxy {
    name: String,
    age: u8,
    character: String,
}

fn main() {
    let me = AIProxy {
        name: "zfk".to_string(),
        age: 26,
        character: "A handsome boy".to_string(),
    };

    let serialized = serde_json::to_string(&me).unwrap();

    println!("serialized = {serialized}");

    let me = serde_json::from_str::<AIProxy>(&serialized).unwrap();

    println!("me = {me:?}");

    let AIProxy { name, .. } = me;

    println!("my name is {name}");

    let data = serde_json::from_reader::<_, serde_json::Value>(
        std::fs::File::open(
            "/Users/zhengfankai/workspace/workflow/study/rust/crates/s-serde/src/data.json",
        )
        .unwrap(),
    )
    .unwrap();

    let name = if let serde_json::Value::String(val) = &data["name"] {
        val.clone()
    } else {
        Default::default()
    };
    println!("data = {data:#?}");
    println!(r###"data["name"] = {}"###, data["name"]);
    println!(r###"data["name"] = {}"###, name);

    let data = serde_json::json!({
      "name": name,
      "age": 43,
      "phones": ["+44 1234567", "+44 2345678"]
    });

    println!("data = {}", serde_json::to_string_pretty(&data).unwrap());
}
