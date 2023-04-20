use std::ptr;

fn main() {
    let mut p: *const i32 = ptr::null();
    let i: i32 = 5;
    p = &i as *const i32;
    let j: i32 = unsafe { *p };

    println!("p: {:p}", p);
    println!("i: {}", i);
    println!("j: {}", j);
}
