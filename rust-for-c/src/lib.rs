#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("hello_from_rust");

    let should_panic = std::env::var("SHOULD_PANIC").unwrap();
    if should_panic == "true" {
        let _ = std::panic::catch_unwind(|| {
            panic!("Error!");
        });
    };
}

// extern "C" fn callback(a: i32) {
//     println!("I'm called from C with value {0}", a);
// }

// #[link(name = "extlib")]
// extern "C" {
//     fn register_callback(cb: extern "C" fn(i32)) -> i32;
//     fn trigger_callback();
// }

// fn main() {
//     unsafe {
//         register_callback(callback);
//         trigger_callback(); // 触发回调
//     }
// }
