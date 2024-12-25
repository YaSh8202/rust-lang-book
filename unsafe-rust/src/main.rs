fn main() {
    // let mut num = 5;

    // let r1 = &num as *const i32; // raw pointer
    // let r2 = &mut num as *mut i32; // mutable raw pointer

    // println!("Hello, world!");

    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }

    let mut v = vec![1, 2, 3, 4, 5, 6];


    let (left, right) = v.split_at_mut(3);

    // let (left, right) = split_at_mut(&mut v, 3);

    println!("{:?}", left);
    println!("{:?}", right);
}

use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    // this is unsafe because we are creating two mutable references to the same data
    // (&mut slice[..mid], &mut slice[mid..])


    // we need to use raw pointers to create two mutable slices
    let ptr = slice.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
