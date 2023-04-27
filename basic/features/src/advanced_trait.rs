use std::{fmt::Display, ops::Add};

pub fn run_advanced_trait() {
    test_advanced_trait();
    test_same_name_case();
    trait_bound_for_trait();
}

fn test_advanced_trait() {
    #[derive(Debug)]
    struct Point {
        x: u32,
        y: u32,
    }

    impl Add<u32> for Point {
        type Output = Self;

        fn add(self, other: u32) -> Self {
            Point {
                x: self.x,
                y: self.y + other,
            }
        }
    }

    impl Add for Point {
        type Output = Self;

        fn add(self, other: Point) -> Self {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let pointer = Point { x: 0, y: 0 };
    let pointer = pointer + Point { x: 1, y: 1 };
    let pointer = pointer + 2;

    println!("added: {:?}", pointer);

    // struct Person<T: Display> {
    //     name: T,
    // }

    // impl<T: Display> Person<T> {}
}

fn test_same_name_case() {
    trait Wizard {
        fn call_name();
        fn fly(&self);
    }
    struct Pilot {}
    impl Pilot {
        fn call_name() {
            println!("pilot call your name");
        }

        fn fly(&self) {
            println!("pilot fly");
        }
    }
    impl Wizard for Pilot {
        fn call_name() {
            println!("wizard call your name");
        }

        fn fly(&self) {
            println!("wizard fly");
        }
    }

    let person = Pilot {};

    person.fly();

    Wizard::fly(&person);

    Pilot::call_name();
    <Pilot as Wizard>::call_name();
}

fn trait_bound_for_trait() {
    trait BorderedDisplay: Display {
        fn print(&self) {
            let text = self.to_string();
            let len = text.len();

            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", text);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Person {
        name: String,
    }
    impl Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, r#"{{ name: {} }}"#, self.name)
        }
    }
    impl BorderedDisplay for Person {}

    Person {
        name: "zhengfankai".to_string(),
    }
    .print();

    // impl Display for Vec<i32> {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, r#"{{ name: {} }}"#, self.name)
    //     }
    // }
}
