#![allow(unused)]

mod bindings;

use bindings::snappy_max_compressed_length;
use libc::{c_int, size_t};

use bindings::snappy_status_SNAPPY_OK;

// extern "C" {
//     fn foo(x: i32, ...);
// }

fn main() {
    let x = unsafe { snappy_max_compressed_length(100) };
    println!("max compressed length of a 100 byte buffer: {}", x);

    println!("snappy_status_SNAPPY_OK = {}", snappy_status_SNAPPY_OK);
}
