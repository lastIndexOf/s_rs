use rand::Rng;
use std::collections::HashMap;

pub fn call() {
    let mut numbers: Vec<i32> = (0..100)
        .map(|_| rand::thread_rng().gen_range(1..=100))
        .collect();

    println!("{:?}", numbers);

    numbers.sort();

    println!("sorted {:?}", numbers);

    let middle_num = numbers.len() / 2;
    let middle_num = if numbers.len() % 2 == 0 {
        middle_num
    } else {
        middle_num + 1
    };

    let mut count_map: HashMap<i32, u8> = HashMap::new();

    for &item in &numbers {
        let count = count_map.entry(item).or_insert(1);
        *count += 1;
    }

    let mut mode_number = 0;
    let mut mode_value = 0;
    for (key, value) in &count_map {
        if *value > mode_number {
            mode_number = *value;
            mode_value = *key;
        }
    }

    println!(
        "middle number is {middle_num} middle value {}",
        numbers.get(middle_num).unwrap_or(&-1)
    );

    println!(
        "count map is {:#?}, mode_number = {mode_number}, mode_value = {mode_value}",
        count_map
    );
}
