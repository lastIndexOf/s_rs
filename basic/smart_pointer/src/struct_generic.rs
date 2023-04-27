use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T: Display>(T);

impl<T: Display + Debug> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        println!("x = {}", x);
        MyBox(x)
    }
}

impl MyBox<String> {
    fn test(&self) {
        println!("test MyBox<String> = {}", self.0);
    }
}

impl<T: Display> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Display> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("drop MyBox, {}", self.0);
    }
}

struct MyStruct<T>(T);

impl MyStruct<String> {
    fn new(x: String) -> Self {
        Self(x)
    }
}

impl<T> MyStruct<Vec<T>> {
    fn new(x: Vec<T>) -> Self {
        Self(x)
    }
}

// impl<T: Display> MyStruct<T> {
//     fn new(x: Vec<T>) -> Self {
//         Self(x)
//     }
// }

pub fn struct_generic_test() {
    let a = 1;
    let b = a;

    let c = vec![1, 2, 3];
    let d = a;
    let e = &c;

    println!("a = {}, b = {}", a, b);
    println!("&a = {:p}, &b = {:p}", &a, &b);
    println!("c = {:?}, c = {:?} &c = {:p}", c, &c, &c);
    println!("d = {}, d = {} &d = {:p}", d, &d, &d);
    println!("e[0] = {}, {}", e[0], e.get(0).unwrap());

    let b = Box::new(5);

    println!("b = {}", b);

    let x = 5;
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    println!("x = {}, y = {}, z = {}", x, y, z);

    test_string_deref(&String::from("hello string"));
    test_vec_deref(&vec![1, 2, 3]);
    test_vec_deref(&[5; 5]);
    // test_str_deref("hello str"); // Error

    let b = MyBox::new(String::from("hello world"));
    let mut c = MyBox::new(String::from("hello world 222"));

    println!("b = {}", *b); // *(b.deref())
    test_string_deref(&b); // b.deref()

    test_string_deref_mut(&mut c);

    drop(b);

    // Box::new(vec![]).test();
    // let c: Box<String> = Box::new(String::from(""));
    // c.test();
    let c = MyStruct::<String>::new(String::from("hello world"));
}

fn test_string_deref(arr: &str) {
    println!("{:?} {:?}", arr, arr.len());
}

fn test_string_deref_mut(arr: &mut str) {
    println!("{:?} {:?}", arr, arr.len());
}

fn test_vec_deref(arr: &[i32]) {
    println!("{:?} {:?}", arr, arr.len());
}
// fn test_str_deref(s: &String) {
//     println!("{:?} {:?}", s, s.len());
// }
