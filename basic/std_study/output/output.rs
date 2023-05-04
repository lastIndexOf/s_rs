#![allow(unused_must_use)]
#![allow(unused)]

use std::{
    borrow::Borrow,
    io::{BufRead, Read, Seek, Write},
    num::{IntErrorKind, ParseIntError},
    ops::{Add, Deref, Div},
};

fn main() {
    s_ptr();
    s_array();
    s_ascii();
    s_borrow();
    s_char();
    s_cmp();
    s_default();
    s_env();
    s_fmt();
    s_fs();
    s_hint();
    s_io();
    s_iter();
    s_marker();
    s_mem();
    s_net();
    s_ops();
    s_option();
    s_os();
    s_path();
}

fn s_ptr() {
    println!(
        "raw pointer {{*const i8}} size is {}",
        std::mem::size_of::<*const i8>()
    );

    println!(
        "raw pointer {{*const u128}} size is {}",
        std::mem::size_of::<*mut u128>()
    );

    println!("type {{u8}} size is {}", std::mem::size_of::<u8>());
}

fn s_array() {
    let arr = std::array::from_fn::<_, 5, _>(|x| x + 1);
    let hello_str = String::from("Hello");
    let arr_hello = std::array::from_ref(&hello_str);

    println!("from fn {:?}", arr);
    println!("from ref {} to {:?}", hello_str, arr_hello);

    assert_eq!(hello_str, arr_hello[0]);
}

fn s_ascii() {
    let asc2 = b'\xd3';
    println!("asc2 = {asc2}");

    for (idx, cr) in std::ascii::escape_default(asc2).enumerate() {
        print!("{asc2} {} char is: {cr}", idx + 1);
    }

    println!("");
}

fn s_borrow() {
    #[derive(Clone)]
    struct MyStrut;

    impl MyStrut {
        fn test(&self) {}
    }

    let s = MyStrut;
    let s_borrow = s.borrow();

    // let s_cloned = s.clone();
    // let s_owned = s.to_owned();

    // let ss = "Hello";
    // let ss_cloned = ss.clone();
    // let ss_owned = ss.to_owned();

    let sce = &[1_u8; 5][..];
    let sce_owned = sce.to_owned();

    let ss = "Hello";
    let ss_owned = ss.to_owned();
    // let ss_owned_cloned = (&ss_owned).clone();

    s.test();
    s_borrow.test();

    println!("slice to owned translate to Vec {:?}", sce_owned);
    println!("&str to owned translate to String {:?}", ss_owned);
}

fn s_char() {
    let from_digit_five = std::char::from_digit(5, 16);
    println!("from_digit_five = {:?}", from_digit_five);
    let from_u32 = std::char::from_u32(1112);
    println!("from_u32_111111111 = {:?}", from_u32);
    println!("from_u32_111111111 = {:?}", std::char::from_u32(2222));
}

fn s_cmp() {
    let x = 2;
    let y = -3;

    assert_eq!(std::cmp::max(x, y), 2);
    assert_eq!(
        std::cmp::max_by(x, y, |x: &i32, y: &i32| x.abs().cmp(&y.abs())),
        -3
    );
    assert_eq!(std::cmp::max_by_key(x, y, |x: &i32| x.abs()), -3);
    assert_eq!(std::cmp::max("A", "a"), "a");
}

fn s_default() {
    let (a, b, (c, d)) = <(i32, u64, (f32, bool)) as Default>::default();
    println!("a = {a}, b = {b}, c = {c}, d = {d}");
}

fn s_env() {
    println!("env::consts::OS = {}", std::env::consts::OS);
    println!("env::consts::ARCH = {}", std::env::consts::ARCH);

    let args = std::env::args().collect::<Vec<String>>();
    let dirname = std::env::current_dir().unwrap().display().to_string();
    let current_file = std::env::current_exe().unwrap().display().to_string();

    // let envs = std::env::vars().collect::<Vec<(String, String)>>();
    let open_ai_key = std::env::var("OPENAI_API_KEY").unwrap();
    // let set_current_dir = std::env::set_current_dir(path)

    let tmp_dir = std::env::temp_dir().display().to_string();

    println!("args = {:?}", args);
    println!("current_dir = {}", dirname);
    println!("current_exe = {}", current_file);
    println!("open_ai_key = {}", open_ai_key);
    println!("temp dir = {}", tmp_dir);
    // println!("envs = {:?}", envs);
}

fn s_fmt() {
    println!("f32::MAX = {}", f32::MAX);
    println!("is nan = {}", f32::is_nan(f32::NAN));
    println!("std::f32::consts = {}", std::f32::consts::PI);
}

fn s_fs() -> std::io::Result<()> {
    // ≈ require('fs').resolve
    let pth = std::fs::canonicalize(".");
    println!("canonicalize path = {:?}", pth);
    let size = std::fs::copy("./src/main.rs", "./output.rs");
    println!("copy ./src/main.rs to ./output.rs, size = {:?}", size);
    let created = std::fs::create_dir_all("./output/output");
    println!("create dir ./output/output result = {:?}", created);
    let stats = std::fs::metadata("./output.rs");
    println!("./output.rs metadata = {:#?}", stats.unwrap().len());

    for dir in std::fs::read_dir("./src")? {
        let dir = dir?.file_name();
        println!("dir = {dir:?}");
    }

    // let dirs = std::fs::read_dir("../")?
    //     .map(|res| res.map(|e| e.file_name()))
    //     .collect::<Result<Vec<_>, std::io::Error>>();

    let dirs = std::fs::read_dir("../")?.flatten().collect::<Vec<_>>();

    println!("dirs = {dirs:?}");

    std::fs::remove_dir("./output").or_else(|err| {
        eprintln!("remove dir error {err}");
        Err(err)
    });

    std::fs::remove_dir_all("./output").and_then(|_| {
        println!("remove dir success!");
        Ok(())
    });

    std::fs::remove_file("./output.rs").and_then(|_| {
        println!("remove file output.rs success!");
        Ok(())
    });

    std::fs::copy("./src/main.rs", "./output.rs").and_then(|_| {
        println!("copy file main.rs to output.rs success!");
        Ok(())
    });

    std::fs::create_dir("./output").unwrap();
    std::fs::rename("./output.rs", "./output/output.rs").or_else(|err| {
        eprintln!("rename file error {err}");
        Err(err)
    });

    std::fs::create_dir_all("./output/output/output");
    std::fs::write("./output/output/output/output.txt", "Hello, World")
        .and_then(|_| {
            println!("write file success!");
            Ok(())
        })
        .or_else(|err| {
            eprintln!("write file error {err}");
            Err(err)
        });

    // println!(
    //     "read content ./output.rs = {}",
    //     String::from_utf8(std::fs::read("./output.rs").unwrap()).unwrap()
    //     std::fs::read_to_string("./output.rs").unwrap()
    // );
    Ok(())
}

fn s_hint() {
    if false {
        unsafe { std::hint::unreachable_unchecked() };
    }

    if false {
        unreachable!();
    }
}

fn s_io() -> std::io::Result<()> {
    let f = std::fs::File::create("./foo.txt")?;
    let mut f = std::io::BufWriter::new(f);
    let size = f.write(&[80])?;
    f.flush();
    std::io::stdout().write(format!("size is {size}\n").as_bytes());

    let f = std::fs::File::open("./foo.txt")?;
    let mut f = std::io::BufReader::new(f);
    let mut line = String::new();
    // let mut content: Vec<u8> = vec![];
    f.read_line(&mut line);
    // let read_size = f.read(&mut content);
    // for line in f.lines() {
    //     let line = line?;
    //     println!("content = {line:?}");
    // }

    // println!("content = {read_size:?}");
    println!("content = {line}");

    let stdout = std::io::stdout();
    let mut stdout = std::io::LineWriter::new(stdout);

    stdout.write_all(b"this is first")?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    stdout.write_all(b"\n")?;

    stdout.write_all(b"please input: ")?;
    stdout.flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    println!("my input is {input}");

    stdout.write_all(b"please input: ")?;
    stdout.flush()?;
    let my_input = std::io::stdin().lock().lines().next().unwrap()?;
    println!("my second input is {my_input}");

    // let (a, b) = std::io::BufReader::new(std::fs::File::open("./output.rs")?)
    //     .chain(std::io::stdin())
    //     .into_inner();

    let mut main_rs = std::fs::File::open("./src/main.rs")?;
    let mut content = [0; 20];

    let size = main_rs.read(&mut content)?;
    println!(
        "main_rs content size = {size}, content = {:?}",
        String::from_utf8(content.to_vec()).unwrap()
    );

    let size = main_rs.read(&mut content)?;
    println!(
        "main_rs content size = {size}, content = {:?}",
        String::from_utf8(content.to_vec()).unwrap()
    );

    main_rs.seek(std::io::SeekFrom::Start(10000));

    let size = main_rs.read(&mut content)?;
    println!(
        "main_rs content size = {size}, content = {:?}",
        String::from_utf8(content.to_vec()).unwrap()
    );

    Ok(())
}

fn s_iter() {
    #[derive(Clone, Copy)]
    struct CounterIter {
        count: usize,
    }
    impl CounterIter {
        fn new() -> Self {
            Self { count: 0 }
        }

        pub const CONST_VAR: i32 = 32;
    }
    impl Iterator for CounterIter {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;

            if self.count <= 5 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = CounterIter::new();
    for count in counter.into_iter() {
        println!("current count is {count}");
    }

    let mut v = vec![1];

    v.get(2);
    let mut v = v.iter_mut();

    println!("v.next() = {:?}", v.next());
    println!("v.next() = {:?}", v.next());
    println!("v.next() = {:?}", v.next());
    println!("v.next() = {:?}", v.next());

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    v1.iter()
        .chain(&v2)
        .for_each(|item| println!("item = {}", *item));

    v1.iter()
        .zip(&v2)
        .for_each(|item| println!("item = {item:?}"));

    let a = [1, 2, 3]
        .iter()
        .filter_map(|&x| if x > 2 { Some(x * 2) } else { None })
        .collect::<Vec<_>>();

    println!("with filter_map = {a:?}");

    let counter = CounterIter::new();

    // let peek_counter = counter.peekable();
    let skip_while_great_3 = counter
        .skip_while(|&item| item < 3 || item == 4)
        .collect::<Vec<_>>();
    println!("skip while greater 3 = {skip_while_great_3:?}");

    let counter = CounterIter::new();
    let take_4 = counter.take(4).collect::<Vec<_>>();
    println!("take 4 = {take_4:?}");

    let counter = CounterIter::new();
    let take_while_less_4 = counter
        .take_while(|&item| item < 4 || item == 5)
        .collect::<Vec<_>>();
    println!("take while less 4 = {take_while_less_4:?}");

    let counter = CounterIter::new();
    let scan = counter
        .scan(<usize as Default>::default(), |p, n| Some(*p + n))
        .collect::<Vec<_>>();

    let counter = CounterIter::new();
    let fold = counter.fold(<usize as Default>::default(), |p, n| p + n);
    println!("scan and fold = {scan:?}, {fold:?}");

    // Result Into Iterator
    let some_vec = Some(vec![1, 2, 3, 4]);
    let some_vec_iter = some_vec.iter();

    for item in some_vec_iter {
        println!("item in {some_vec:?} is {item:?}");
    }
    for item in <Option<u32> as IntoIterator>::into_iter(None) {
        // None 转化为迭代器后表示没有值
        println!("item in {some_vec:?} is {item:?}");
    }

    let flatten = [Some(1), Some(2), None, Some(3)]
        .iter()
        .flatten()
        .collect::<Vec<_>>();

    println!("Result list to iter list = {flatten:?}");

    let fuse = [Some(1), Some(2), None, Some(3)]
        .iter()
        .fuse()
        .inspect(|&x| {
            if x.is_none() {
                println!("meet a None");
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    println!("fuse {{Some(1), Some(2), None, Some(3)}} result is {fuse:?}");

    struct Alternate {
        state: i32,
    }

    impl Iterator for Alternate {
        type Item = i32;

        fn next(&mut self) -> Option<i32> {
            let val = self.state;
            self.state = self.state + 1;

            // if it's even, Some(i32), else None
            if val % 2 == 0 {
                Some(val)
            } else {
                None
            }
        }
    }

    let fused_iter = Alternate { state: 2 };

    for item in fused_iter {
        println!("fused item = {item}");
    }

    let mut fused_iter = Alternate { state: 2 }.take(4);
    println!("fused_iter_item = {:?}", fused_iter.next());
    println!("fused_iter_item = {:?}", fused_iter.next());
    println!("fused_iter_item = {:?}", fused_iter.next());
    println!("fused_iter_item = {:?}", fused_iter.next());
    println!("fused_iter_item = {:?}", fused_iter.next());
    println!("fused_iter_item = {:?}", fused_iter.next());

    let counter = CounterIter::new();
    println!(
        "counter size = {:?}",
        counter.step_by(2).collect::<Vec<_>>()
    );

    // node.js find, findIndex, indexOf, lastIndexOf
    let counter = CounterIter::new();
    let first_2 = counter.clone().find(|x| *x == 2);
    let first_2_index = counter.clone().position(|x| x == 2);
    let r_first_2_index = [1, 2, 3].iter().rposition(|x| *x == 3);

    println!(
        "first_2 = {:?}, first_2_index = {:?}, r_first_2_index = {:?}",
        first_2, first_2_index, r_first_2_index,
    );

    println!("2.4 and 3.2 max is {}", f32::max(2.4, 3.2));
    println!("2.4 and NAN max is {}", f32::max(2.4, f32::NAN));

    // 比较浮点数的最大最小没法用 max, min 因为浮点数没有实现 Ord, 可以采用 reduce + f32::man / f32:: min
    // let min = [2.4, f32::NAN, 3.2]
    //     .into_iter()
    //     .reduce(f32::max)
    //     .ok_or(0.0_f32)
    //     .unwrap();

    let min = [2.4, f32::NAN, 3.2]
        .iter()
        .copied()
        // .cloned()
        .reduce(f32::max)
        .ok_or(0.0_f32)
        .unwrap();
    println!("min = {min}");

    let sum = [1.2, 2.3, 4.3, 5.4].iter().sum::<f32>();
    println!("sum is {sum}");

    let a = [1, 2, 3, 4, 5];
    let b = [1, 2, 3, 4, 6];
    let c = [3, 4, 6];
    let d = [0, 1, 2];
    let e = [1, 2, 3, 0, 0, 0];
    let f = [1, 2, 3, 0, 0, 0, 0];

    let cmp_less = a.iter().cmp(b.iter());
    let cmp_great = c.iter().cmp(a.iter());
    let eq = e.iter().eq(f.iter());
    let ne = a.iter().ne(d.iter());
    let less = b.iter().le(c.iter());
    let great = c.iter().ge(a.iter());

    println!("less = {cmp_less:?}");
    println!("great = {cmp_great:?}");
    println!("eq = {eq}");
    println!("ne = {ne}");
    println!("less = {less}");
    println!("great = {great}");

    println!("1 + 2 + 3 + 4 + 5 + 6 = {}", (1..=6).sum::<usize>());
    println!("1 * 2 * 3 * 4 * 5 * 6 = {}", (1..=6).product::<usize>());

    let _dirs = std::fs::read_dir("../")
        .unwrap()
        .map(|dir| dir.map(|entry| entry.path()))
        .flatten()
        // .chain(std::iter::once(std::path::PathBuf::from("~/.config")))
        .chain(std::iter::once_with(|| {
            std::path::PathBuf::from("~/.config")
        }))
        .collect::<Vec<_>>();

    let _dirs = std::fs::read_dir("../")
        .unwrap()
        .map(|dir| dir.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>();
}

fn s_marker() {
    struct NoneCopy {}

    let some = Some(5);
    let _some_copied = some;
    let _some_copied_twice = some;

    let none_copy_some = Some(NoneCopy {});
    let _none_copy_some_moved = none_copy_some;
    // following will panic because of borrowing
    // let none_copy_some_moved_twice = none_copy_some;

    enum CopiedEnum {
        Member1(i32),
        Member2,
    }
    let one = CopiedEnum::Member1(32);
    let two = CopiedEnum::Member2;

    let _one_copied = one;
    // let one_copied = one; // Error
    let _two_copied = two;
    let _two_copied = CopiedEnum::Member2;
}

fn s_mem() {
    #[derive(Debug)]
    struct StringPtr {
        _ptr: String,
    }

    let str_ptr = StringPtr {
        _ptr: String::new(),
    };

    println!(
        "{str_ptr:?} align of is {}",
        std::mem::align_of_val(&str_ptr)
    );

    println!("String align of is {}", std::mem::align_of::<String>());
    println!("sizeof ptr is {}", std::mem::size_of::<&String>());

    enum DiscriminateEnum {
        EnumA,
        EnumB,
    }

    assert_eq!(
        std::mem::discriminant(&DiscriminateEnum::EnumA),
        std::mem::discriminant(&DiscriminateEnum::EnumA)
    );

    assert_ne!(
        std::mem::discriminant(&DiscriminateEnum::EnumB),
        std::mem::discriminant(&DiscriminateEnum::EnumA)
    );

    let ptr = {
        let ptr = Box::leak(Box::new(String::from("Hello, World")));
        ptr
    };

    (*ptr).extend("with extend".chars());
    println!("ptr = {}", *ptr);

    unsafe {
        let _ = Box::from_raw(ptr);
        println!(
            "ptr = {ptr:p}, ptr.as_ptr() = {:p}, ptr as *mut String = {:p}",
            ptr.as_ptr(),
            ptr as *mut String
        );
    }

    println!("ptr = {}", *ptr);

    let vector = vec![1, 2, 3];

    println!(
        "&vector = {:p}, &vector as *mut Vec = {:p}, vector.as_ptr() = {:p}",
        &vector,
        &vector as *const Vec<i32>,
        vector.as_ptr()
    );

    let string = String::from("hello world");

    println!(
        "&string = {:p}, &string as *mut Vec = {:p}, string.as_ptr() = {:p}",
        &string,
        &string as *const String,
        (&string).as_ptr()
    );

    println!("Box<i32> ptr = {:p}", Box::new(32_i32).deref());

    let mut mem_ops_vec = vec![1, 2, 3, 4, 5];

    std::mem::replace(&mut (mem_ops_vec[2]), 30);
    println!("after replace vec = {mem_ops_vec:?}");

    std::mem::take(&mut mem_ops_vec[2]);
    println!("after take vec = {mem_ops_vec:?}");

    std::mem::swap(&mut mem_ops_vec, &mut vec![1]);
    println!("after swap vec = {mem_ops_vec:?}");
}

fn s_net() {
    println!("(1..2).start = {}", (1..2).start);
    println!("(1..2).end = {}", (1..2).end);
    println!("(1..=2).end = {:?}", (1..=2));

    let a = [1, 2, 3];
    let b = a
        .iter()
        .copied()
        .chain(std::iter::once(5))
        .collect::<Vec<_>>();

    let ipv4_addr = "0.0.0.0".parse::<std::net::Ipv4Addr>();
    let socket_addr_v4 = "0.0.0.0:8080".parse::<std::net::SocketAddrV4>();

    // ipv4_addr.unwrap().octets();
    // socket_addr_v4.unwrap().ip();
    // socket_addr_v4.unwrap().port();
    // std::net::Ipv4Addr::LOCALHOST
    // std::net::Ipv6Addr::LOCALHOST

    println!("ipv4_addr = {ipv4_addr:?} socket_addr_v4 = {socket_addr_v4:?}");
}

fn s_ops() {
    let range = (1.0_f64..2.0_f64);
    // Error
    // for i in (1.0_f64..2.0_f64) {
    //     println!("i = {i}");
    // }

    println!("range contains 1.5 = {}", range.contains(&1.5));

    // Infinity
    println!(
        "infinity range (1..) take 5 = {:?}",
        (1..).take(10).collect::<Vec<_>>()
    );
}

fn s_option() {
    println!("4.0 / 2.0 = {}", 4.0.div(2.0));
    println!("4.0 / 0.0 = {}", 4.0.div(0.0));

    assert_eq!(f32::INFINITY, 4.0 / 0.0);

    let a = "-91282223472332".parse::<i32>().or_else(|err| {
        println!("err = {err}");
        Err(err)
    });

    println!("a = {a:?} i16::MIN = {}", i16::MIN);

    // fn make_iter(do_insert: bool) -> impl Iterator<Item = i32> {
    //     // Explicit returns to illustrate return types not matching
    //     match do_insert {
    //         true => return (0..4).chain(std::iter::once(42)).chain(4..8),
    //         false => return (0..4).chain(std::iter::empty::<i32>()).chain(4..8),
    //     }
    // }
    fn make_iter(do_insert: bool) -> impl Iterator<Item = i32> {
        match do_insert {
            true => return (0..4).chain(Some(42)).chain(4..8),
            false => return (0..4).chain(None).chain(4..8),
        }
    }

    let mut some_string = Some(String::from("hello world"));
    println!("some_string = {some_string:?}");

    some_string.replace(String::from("changed"));
    println!("some_string = {some_string:?}");

    *(&mut some_string) = None;
    println!("some_string = {some_string:?}");
}

fn s_os() {}

fn s_path() {
    let app_directory = std::env::current_dir();
    let home = std::env::var("HOME");

    println!("app_directory = {app_directory:?}, home = {home:?}");

    let main = std::path::Path::new("./src/main.rs");

    println!("main = {main:?}");
    println!("main.file_stem = {:?}", main.file_stem());
    println!("main.file_name = {:?}", main.file_name());
    println!("main.extension = {:?}", main.extension());

    let ancestors = main.ancestors().collect::<Vec<_>>();
    println!("main.ancestors = {ancestors:?}");

    assert_eq!(
        main.strip_prefix("./"),
        Ok(std::path::Path::new("src/main.rs"))
    );

    println!(
        "{{./src/main.rs}}.canonicalize = {:?}",
        std::path::Path::new("./src/main.rs").canonicalize()
    );

    println!("{{./src/main.rs}}.parent = {:?}", main.parent());

    println!(
        "{{./src/main.rs}}.parent = {:?}",
        std::path::Path::new("./src/main.rs")
            .canonicalize()
            .unwrap()
            .parent()
    );

    println!(
        "./main.rs is exists = {}",
        std::path::Path::new("./main.rs").exists()
    );

    println!(
        "./main.rs is dir = {}",
        std::path::Path::new("./main.rs").is_dir()
    );

    // alias to std::fs::metadata
    println!(
        "./src/main.rs metadata = {:?}",
        std::path::Path::new("./src/main.rs").metadata()
    );

    let canonicalize = main.canonicalize().unwrap();
    let without_app_dir = canonicalize.strip_prefix(std::env::current_dir().unwrap());

    let current_dir = std::env::current_dir().unwrap();
    let pth = <std::path::PathBuf as AsRef<std::path::Path>>::as_ref(&current_dir);

    println!("without_app_dir = {without_app_dir:?}, pth = {pth:?}");
}
