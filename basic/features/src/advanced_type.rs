use std::collections::HashMap;

pub fn run_advanced_type() {
    struct Person(HashMap<String, i32>);
    impl Person {
        fn new() -> Self {
            Self(HashMap::new())
        }
        fn set(&mut self, name: String, id: i32) {
            self.0.insert(name, id);
        }

        fn get_by_name(&self, name: &str) -> Option<&i32> {
            self.0.get(name)
        }
    }

    let mut me = Person::new();

    me.set("zhengfankai".to_string(), 12345);

    println!("me: {:?}", me.get_by_name("zhengfankai").unwrap());

    let arr = vec![1, 2, 3];
    let arr = arr
        .iter()
        // .map(|item| item.to_string())
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    println!("arr: {:?}", arr);

    #[derive(Debug)]
    enum Status {
        Value(u32),
        // Stop,
    }

    let lst = (0u32..20).map(Status::Value).collect::<Vec<Status>>();

    println!("list: {:?}", lst);

    println!("return_closure: {}", returns_closure().0(12));
}

fn returns_closure() -> (Box<dyn Fn(i32) -> i32>, fn(i32) -> i32) {
    // let arr: Vec<Box<dyn Fn(i32) -> i32>> =
    //     vec![Box::new(|x: i32| x + 2), Box::new(|x: i32| x + 3)];

    (Box::new(|x| x + 1), test)
}

fn test(x: i32) -> i32 {
    x
}
