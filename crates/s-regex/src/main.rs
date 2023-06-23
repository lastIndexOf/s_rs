use chrono::Datelike;
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

lazy_static! {
    static ref PHONE_NUMBER: Regex = Regex::new(r#"(?P<phone>1[3-9]\d{9})"#).unwrap();
    static ref SOME_CHINESE: Regex = Regex::new(r#"我"#).unwrap();
}

fn main() {
    println!(
        "13168792900 is match {}",
        PHONE_NUMBER.is_match("13168792900")
    );

    for phone in PHONE_NUMBER.captures_iter("My phone number is 13168792900, her is 15915491300") {
        println!("phone = {:?}", phone.name("phone").unwrap().as_str());
    }

    let res = PHONE_NUMBER
        .replace_all("My number is 13168792900, but her is 15915491300", "911")
        .to_string();

    println!("res = {res}");

    println!(
        "我的名字是 is match {}",
        SOME_CHINESE.is_match("我的名字是")
    );

    let random_num = rand::thread_rng().gen_range(0.0..=1.0);
    println!("a random number between 10 - 100 is {random_num}");

    let now = chrono::Utc::now();
    let now_local = chrono::Local::now();

    println!("now is {}", now.to_string());
    println!("now loc is {}", now_local.to_string());

    let format_time = now_local.format("%Y-%m-%d %H:%M:%S");
    println!("now loc is {}", format_time.to_string());

    let before_30_d = chrono::Utc::now() - chrono::Duration::days(30);
    println!("before_30_d = {before_30_d}");
}
