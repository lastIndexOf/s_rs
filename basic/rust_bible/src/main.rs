use rust_bible::{test_in_lib, MyFuture};
use std::{error::Error, fs, ops::Deref};

use futures::{executor::block_on, future::join_all, join, select, try_join, FutureExt};

use crate::{bro_2::print_in_bro_2, lifetime::test_lifetime};

mod bro;
mod bro_2;
mod lifetime;

#[derive(Debug)]
struct Foo;

impl Foo {
    fn mutate_and_share(&mut self) -> &Self {
        &*self
    }
    fn _share(&self) {}
}
fn main() {
    let _ = basic();

    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    // foo._share();
    println!("{:?}", loan);

    self_ref();
    print_in_bro_2();

    block_on(do_some_async_things());
    test_in_lib();
    println!("Hello, world! {} {}", bro::BRO_1, bro::BRO_1);

    test_lifetime();

    block_on(async {
        println!("before 2s ...");
        MyFuture::new(true).await;
        println!("after 2s ...");
    });

    block_on(async {
        let timer_1 = MyFuture::new(false);
        let timer_2 = MyFuture::new(false);
        let timer_3 = MyFuture::new(false);

        let (a, b, c) = join!(timer_1, timer_2, timer_3);

        match join_all([MyFuture::new(false), MyFuture::new(false)]).await[..] {
            [a, b] => println!("a = {:?}, b = {:?}", a, b),
            _ => println!("empty"),
        }

        println!("a = {:?} {:?} {:?}", a, b, c);
        let mut a = MyFuture::new(true).fuse();
        let mut b = MyFuture::new(false).fuse();

        select! {
            x = a => println!("in select x = {:?}", x),
            y = b => println!("in select y = {:?}", y),
            // complete => println!("in select complete"),
            // default => panic!()
        }
    })
}

fn self_ref() {
    let a: *mut () = &mut ();

    unsafe {
        println!("a = {:p}", a);
        *a = ().clone();
        println!("a = {:p}", a);
    };

    let a = "hello".to_string();

    println!("b = {:p}", &a);
    {
        let b = &a;

        println!("b = {:p}", b);
    }
}

async fn do_some_async_things() {
    println!("go go go!");

    do_some_thing_after_async().await;
}

async fn do_some_thing_after_async() {
    println!("I should execute moment later");
}

fn basic() -> Result<(), Box<dyn Error>> {
    let f_abs = 3.14f32.round();

    println!("f_abs = {}", f_abs);

    println!("s = {}", "nice work".to_owned());
    println!("s = {}", "nice weather".replace(" ", ", "));
    println!("s = {}", "nice weather".to_lowercase());

    for _ in 1..1 {
        println!("called");
    }

    let content = fs::read_to_string("./Cargo.toml")?;

    println!("content = {content}");

    let v = (1..5).map(|x| x).collect::<Vec<i32>>();

    // println!("content = {}", "hello".split(" ").collect<Vec<&str>>());
    match &v[..] {
        [first @ 1..=3, ..] => println!("first = {} 222", *first),
        [_, _, _, e] => println!("first = {}", e),
        [first, .., last] => println!("first = {}, last = {}", first, last),
        _ => println!("empty"),
    }

    #[derive(Debug)]
    struct Person {
        name: String,
    }
    impl AsRef<String> for Person {
        fn as_ref(&self) -> &String {
            &self.name
        }
    }
    impl Deref for Person {
        type Target = String;
        fn deref(&self) -> &Self::Target {
            &self.name
        }
    }

    let p = Person {
        name: "zfk".to_string(),
    };

    fn print_person(p: &String) {
        println!("p = {:?}", p);
    }

    println!(
        "multiple trait with same fn: {:?}, {}, {}",
        <String as AsRef<str>>::as_ref(&"ssss".to_string()).chars(),
        "32".parse::<i32>().unwrap(),
        "32".parse::<i32>().map(|x| x + 2).unwrap()
    );
    //
    // "sss".chars()
    // let it = <&str as Into<String>>::into("into test");

    // "32".parse::<i32>().map(|x| x + 2).ok()

    print_person(&p);

    // let v = vec![1, 2, 3];

    let a = 'a' as u32;

    println!("a = {}", a);

    Ok(())
}
