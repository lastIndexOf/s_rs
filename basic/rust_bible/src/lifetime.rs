#![allow(unused)]
#![allow(unused_imports)]

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    ops::Add,
    ptr::NonNull,
    rc::Rc,
    sync::{atomic::Ordering, Arc, Mutex},
    thread,
};

use futures::task::UnsafeFutureObj;

struct Interface<'a: 'b, 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'a, 'b> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b> {
        Interface {
            manager: &mut self.manager,
        }
    }
}

pub fn test_lifetime() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);

    let r3: &str;

    {
        let s1 = "String".to_string();

        // s1 虽然没有 'static 生命周期，但是它依然可以满足 T: 'static 的约束
        // 充分说明这个约束是多么的弱。。
        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
        // r3 = &s1;

        // s1 在这里被 drop
    }
    // println!("{}", r3);

    let v = vec![1, 2, 3];
    let i = v.iter();
    let s = i.sum::<i32>();

    println!("{}", s);
    println!("{:?}", v);

    // for item in v.iter() {}

    let v = vec![1, 2, 3];
    let i = v.into_iter();

    // for item in i {}

    let names = ["sunface", "sunfei"];
    let ages = [18, 20];
    let folks = names
        .into_iter()
        .zip(ages.into_iter())
        .collect::<HashMap<&str, i32>>();

    println!("{:?}", folks);

    let data = folks.iter().fold(0, |acc, (_, age)| acc + age);
    println!("data = {:?}", data);

    println!(
        "data = {:?}",
        folks.into_iter().reduce(|a, b| {
            println!("{:?}", a);
            println!("{:?}", b);
            (a.0, a.1 + b.1)
        })
    );

    let mut v = vec![1, 2, 3];
    let p = v.as_mut_ptr();
    let n = p as usize;
    let n = n + std::mem::size_of::<i32>();
    let n = n as *mut i32;
    unsafe {
        *n += 10;
    }
    println!("{:?}", v);

    #[derive(Clone, Debug)]
    struct Container<T>(Arc<T>);

    fn clone_containers<T: Debug>(
        foo: &Container<i32>,
        bar: &Container<T>,
        ccc: Container<i32>,
        ddd: Container<T>,
    ) {
        // let a = (*bar);
        let foo_cloned = foo.clone();
        let bar_cloned = bar.clone();
        let ccc_cloned = ccc.clone();
        // let ddd_cloned = ddd.clone();

        println!("aaaaaaaaa{:?}", foo_cloned);
        println!("{:?}", bar_cloned);
        // let ccc_cloned = ccc.clone();
        // (*bar).0 = foo.0.clone();
    }

    struct MyStruct;

    impl MyStruct {
        fn foo(&self) {}
        fn bar(self) -> Self {
            self
        }
    }

    let mut m = MyStruct;
    m = m.bar();
    m.foo();
    let r = &mut m;
    r.foo();
    // let a = m;
    // let a = *r;
    println!("m = {:p}", &m);

    let a = 5;
    let b = &a;
    let c = &(*b);

    println!("a pointer = {:p}, &*&a pointer = {:p}", &a, c);

    // clone_containers(&Container(Arc::new(1)), &Container(Arc::new("sss")));

    let a = Container(Arc::new(1));
    let b = a;

    let mut s = String::new();
    let s = (&s).clone();
    let b = Box::new(String::new());
    let b = (&b).clone();

    fn do_stuff_1<T: Clone>(value: &T) {
        let cloned = value.clone();
    }

    fn do_stuff<T>(value: &T) {
        let cloned = value.clone();
    }

    let array: Rc<Box<[i32; 3]>> = Rc::new(Box::new([1, 2, 3]));
    let first_entry = array[0];
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);

    let a = Some(String::new());

    let b = a;

    #[derive(Clone, Copy)]
    struct MyStruct {
        val: i32,
    }

    struct MyNode {
        val: String,
    }

    let a = MyStruct { val: 2 };
    let b = a;
    let c = a;

    let a = Box::new(32i32);
    let b = a;
    // let c = a;

    let a = Rc::new(32i32);
    let b = a;
    // let c = a;

    let a = RefCell::new(32i32);
    let b = a;
    // let c = a;

    let a = Some(32i32);
    let b = a;
    let c = a;

    let a = Some(MyStruct { val: 32i32 });
    let b = a;
    let c = a;

    let a = Some(unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(String::new()))) });

    let b = a;
    let c = a;

    fn test() -> String {
        let a = Box::new(MyNode { val: String::new() });

        {
            let b = a.val;
            return b;
        }

        // println!("{}", a.val);

        // a.val
    }

    enum MyEnum {
        A = 1,
        B,
        C,
    }
    impl TryFrom<i32> for MyEnum {
        type Error = String;
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            Ok(Self::A)
        }
    }

    let a = <i32 as TryInto<MyEnum>>::try_into(111);

    struct MyNode2 {
        value: i32,
    }

    let a = Box::new(32i32);
    let b = a;

    // println!("a = {a}");

    let n = MyNode2 { value: 1 };
    let b = n;

    let a = 32i32;
    let p = &a;
    let p2 = p;
    let p3 = p;
    // let c = n;

    let a = Box::new(String::new());

    let c = a.chars();
    let c = (&a).chars();

    let s = "hello";
    let s2 = s;
    let s3 = s;

    fn is_even(i: i32) -> bool {
        false
    }

    fn retain_even(nums: &mut Vec<i32>) {
        let mut i = 0;
        for num in nums.iter().filter(|&num| is_even(*num)) {
            // nums[i] = *num;
            i += 1;
        }
        nums.truncate(i);
    }
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        pointer_to_value: *const String,
    }

    impl SelfRef {
        fn new(txt: &str) -> Self {
            SelfRef {
                value: String::from(txt),
                pointer_to_value: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.value;
            self.pointer_to_value = self_ref;
        }

        fn value(&self) -> &str {
            &self.value
        }

        fn pointer_to_value(&self) -> &String {
            assert!(
                !self.pointer_to_value.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &(*self.pointer_to_value) }
        }
    }

    let mut a = vec![1, 2, 3];
    let mut x = 0;

    std::thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // We can borrow `a` here.
            dbg!(&a);
        });
        s.spawn(|| {
            println!("hello from the second scoped thread");
            // We can even mutably borrow `x` here,
            // because no other threads are using it.
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());

    let a = test_option_logic();
    let a = test_result_logic();
}

fn test_option_logic() -> Option<i32> {
    let o1 = Some(1);
    let o2 = Some(2);

    let n = None;

    o1.or(o2); // o1
    o1.or(n); // o1
    n.or(o1); // o1

    o1.and(o2); // o2
    o1.and(n); // n

    // o1.filter(predicate)
    // o1.map(f)
    // o1.ok_or(err)

    n
}

fn test_result_logic() -> Result<i32, i32> {
    let s1: Result<i32, i32> = Ok(1);
    let s2: Result<i32, i32> = Ok(2);
    let e: Result<i32, i32> = Err(1);

    let a = s1.or(e); // s1
    let a = s1.or(s2); // s1
    let a = s1.or(e); // s1
    let a = e.or(s2); //s2
    let a = s1.and(s2); // s2
    let a = s1.and(e); // e
    let a = e.and(s1); //e

    let a = s1.or_else(|_| Err(2));
    // let a = s1.and_then(|_| Err(2));

    // let a = s1

    // e.map(op);
    // e.map_err(op);
    // e.map_or(op);
    // e.map_or_else(default, f);

    // e.ok

    let a = 1usize;
    let p = a as *mut String;

    use std::{slice::from_raw_parts, str::from_utf8_unchecked};

    fn get_ptr() -> (usize, usize) {
        // let s = "hello world".to_string();
        let s = "你好世界";
        let ptr = s.as_ptr();
        let length = s.len();
        (ptr as usize, length)
    }

    fn from_ptr((pointer, length): (usize, usize)) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    println!("from_ptr(get_ptr()) = {}", from_ptr(get_ptr()));
    println!("from_ptr(get_ptr()) = {}", get_ptr().1);
    println!("from_ptr(get_ptr()) = {}", "h".as_bytes().len());
    println!("from_ptr(get_ptr()) = {}", "你".as_bytes().len());

    let a = Box::new(String::new());
    let b = Box::into_raw(a);

    extern "C" {
        fn abs(x: i32) -> u32;
    }

    println!("C FFI: abs(-32) = {}", unsafe { abs(-32i32) });

    use std::sync::Barrier;

    let mut threads = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));
    let ss = Arc::new(Mutex::new(String::new()));

    for _ in 0..6 {
        let barrier = Arc::clone(&barrier);
        let ss = Arc::clone(&ss);
        threads.push(thread::spawn(move || {
            println!("before",);
            *(ss.lock().unwrap()) = String::from("Hello");
            barrier.wait();
            println!("after");
        }))
    }

    for t in threads {
        t.join().unwrap();
    }

    println!("ss = {}", ss.lock().unwrap());

    let a = (1, 2);
    let b = a;
    let c = a;

    let a = (String::new(), 2);
    let b = a;
    // let c = a;

    let val = Mutex::new(5);

    {
        let mut val = val.lock().unwrap();
        *val = 6;
    }

    println!("val.lock().unwrap() = {:?}", val.lock().unwrap());

    // Ordering

    e
}
