fn main() {
    use std::slice;

    let test_array = vec![42];
    let pointer = test_array.as_ptr();
    let r = pointer as *mut i32;

    println!("{:?}", r);

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    println!("{:?}", values);
}
