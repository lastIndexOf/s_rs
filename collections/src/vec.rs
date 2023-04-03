pub fn create_vec_v1() -> Vec<i32> {
    Vec::new()
}

pub fn create_vec_v2(initial: i32, len: usize) -> Vec<i32> {
    vec![initial; len]
}

// pub fn each(vec: &Vec<i32>) {
//     for item in vec {
//         println!("item is {item}");
//     }
// }

// pub fn add_one(vec: &mut Vec<i32>) {
//     for item in vec {
//         *item += 1;
//     }
// }
