use futures::{
    task::{Context, Poll},
    FutureExt,
};
use std::{
    future::Future,
    marker::PhantomPinned,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{ready, Waker},
    thread,
    time::Duration,
};

pub struct MyFuture {
    shared_state: Arc<Mutex<SharedState>>,
    timer: Option<Box<MyFuture>>,
}

struct SharedState {
    complete: bool,
    waker: Option<Waker>,
}

impl MyFuture {
    pub fn new(has_timer: bool) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            complete: false,
            waker: None,
        }));

        let shared_state_cloned = shared_state.clone();
        thread::spawn(move || {
            println!("in child thread");

            thread::sleep(Duration::from_secs(if has_timer { 5 } else { 2 }));

            let mut shared_state = shared_state_cloned.lock().unwrap();
            shared_state.complete = true;
            shared_state.waker.take().unwrap().wake();
        });

        Self {
            shared_state,
            timer: if has_timer {
                Some(Box::new(MyFuture::new(false)))
            } else {
                None
            },
        }
    }
}

impl Future for MyFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("in timer poll");

        if let Some(timer) = self.timer.as_mut() {
            ready!(timer.poll_unpin(cx));
        }

        let mut shared_state = self.shared_state.lock().unwrap();
        match shared_state.complete {
            true => Poll::Ready(()),
            false => {
                shared_state.waker = Some(cx.waker().clone());
                Poll::Pending
            }
        }
    }
}

pub fn test_in_lib() {
    struct SelfRef {
        value: String,
        ptr: *const String,
        _marker: PhantomPinned,
    }

    impl SelfRef {
        fn new(value: &str) -> Self {
            let value = value.to_string();
            // let ptr = &value as *const String;

            let mut res = Self {
                value,
                ptr: std::ptr::null(),
                _marker: PhantomPinned,
            };

            res.ptr = &res.value as *const String;

            res
        }

        fn value(&self) -> &str {
            &self.value
        }

        fn value_(&self) -> &str {
            assert!(!self.ptr.is_null());
            unsafe { &(*self.ptr) }
        }
    }

    let mut a = SelfRef::new("aaa");
    let mut b = SelfRef::new("bbb");

    let mut a = unsafe { Pin::new_unchecked(&mut a) };
    let mut b = unsafe { Pin::new_unchecked(&mut b) };

    println!("a = {}, {}", a.value(), a.value_());
    println!("b = {}, {}", b.value(), b.value_());

    std::mem::swap(&mut a, &mut b);
    // std::mem::swap(a.get_mut(), b.get_mut());

    println!("a = {}, {}", a.value(), a.value_());
    println!("b = {}, {}", b.value(), b.value_());

    let a = String::from("hello");

    println!("a.addr = {:p}, a.ptr = {:p}", &a, a.as_ptr());
    let b = a;
    println!("b.addr = {:p}, b.ptr = {:p}", &b, b.as_ptr());

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned, // 这个标记可以让我们的类型自动实现特征`!Unpin`
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }

        fn a(self: Pin<&Self>) -> &str {
            &self.get_ref().a
        }

        fn b(self: Pin<&Self>) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.b) }
        }
    }

    // 此时的`test1`可以被安全的移动
    let mut test1 = Test::new("test1");
    // 新的`test1`由于使用了`Pin`，因此无法再被移动，这里的声明会将之前的`test1`遮蔽掉(shadow)
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!(
        "a: {}, b: {}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );
    // std::mem::swap(test1.get_mut(), test2.get_mut());
    println!(
        "a: {}, b: {}",
        Test::a(test2.as_ref()),
        Test::b(test2.as_ref())
    );
}
