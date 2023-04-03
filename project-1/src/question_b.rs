const Vowel: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn call() {
    let mut text = "hello world".to_string();
    let mut words = text.split_whitespace();

    let mut result = String::new();
    for word in &mut words {
        match word.find(|char| !Vowel.contains(&char)) {
            Some(idx) => {
                print!("idx = {idx}");

                let new_word = if idx == 0 {
                    format!("{}-{}ay", &word[1..], word.chars().nth(idx).unwrap())
                } else if idx == word.len() {
                    format!("{}-{}ay", &word[..idx], word.chars().nth(idx).unwrap())
                } else {
                    format!(
                        "{}{}-{}ay",
                        &word[..idx],
                        &word[(idx + 1)..],
                        word.chars().nth(idx).unwrap()
                    )
                };

                result.push_str(" ");
                result.push_str(&new_word);
            }
            None => println!("{word} 全部是元音字母"),
        }
    }

    println!("转化后 {result}");
}
