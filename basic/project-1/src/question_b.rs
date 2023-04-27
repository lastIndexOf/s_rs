const VOWEL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn call() {
    let text = "hello world";
    let words = text.split_whitespace();

    let mut result = String::new();
    for word in words {
        match word.find(|char| !VOWEL.contains(&char)) {
            Some(idx) => {
                print!("idx = {idx}");

                let new_word = match idx {
                    0 => format!("{}-{}ay", &word[1..], word.chars().nth(idx).unwrap()),
                    idx if idx == word.len() => {
                        format!("{}-{}ay", &word[..idx], word.chars().nth(idx).unwrap())
                    }
                    _ => format!(
                        "{}{}-{}ay",
                        &word[..idx],
                        &word[(idx + 1)..],
                        word.chars().nth(idx).unwrap()
                    ),
                };

                result.push_str(" ");
                result.push_str(&new_word);
            }
            None => println!("{word} 全部是元音字母"),
        }
    }

    println!("转化后 {result}");
}
