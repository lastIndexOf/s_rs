use std::slice;

pub fn run_unsafe_rust() {
    test_raw_pointer();
    test_abstract();

    test_unsafe_mem();
    test_extern_fn();
}

fn test_raw_pointer() {
    let mut x = String::from("Hello");

    let ptr_a = &x as *const String;
    let ptr_b = &mut x as *mut String;

    unsafe {
        println!("ptr_a: {}", *ptr_a);
        println!("ptr_a: {}", *ptr_b);
    }
}

fn test_abstract() {
    let mut arr = vec![1, 2, 3, 4, 5];
    // let (left, right) = arr.split_at_mut(2);
    let (left, right) = split_at_mut(&mut arr, 2);

    println!("left: {:?}", left);
    println!("right: {:?}", right);

    assert_eq!(left, &mut [1, 2]);
    assert_eq!(right, &[3, 4, 5]);
}

fn test_unsafe_mem() {
    // let address = 0x012345usize;
    // let address = address as *mut i32;

    // let res = unsafe { slice::from_raw_parts_mut(address, 10000) };

    // println!("res: {:?}", res);
}

fn test_extern_fn() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

#[cfg(test)]
mod test_unsafe_rust {
    use super::*;

    #[test]
    fn test_run_unsafe_rust() {
        run_unsafe_rust();

        assert_eq!(1, 1);
    }
}
