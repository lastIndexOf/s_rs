use std::collections::HashMap;

#[derive(Debug)]
enum Data {
    Null,
    String(String),
}

pub fn create_hash_map() {
    let mut map = HashMap::new();

    map.insert("name".to_string(), Data::String("zhengfankai".to_string()));
    map.insert("age".to_string(), Data::String("27".to_string()));
    map.insert(
        "email".to_string(),
        Data::String("kgaikj2cu@icloud.com".to_string()),
    );

    for (key, value) in &map {
        println!("key is {key}, value is {:#?}", value);
    }

    map.entry("phone".to_string())
        .or_insert(Data::String("13168792900".to_string()));
    let phone = map
        .entry("phone".to_string())
        .or_insert(Data::String("18588973971".to_string()));

    let phone = match phone {
        Data::String(val) => val,
        _ => panic!("phone is not a string"),
    };

    phone.push('!');

    let email = map.get(&String::from("email")).unwrap_or(&Data::Null);

    println!("email is {:#?}", email);

    println!("user is {:#?}", map);
}
