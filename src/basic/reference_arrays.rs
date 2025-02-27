#![allow(dead_code)]
pub fn arr_reference() {
    let arr = [2, 3, 4, 5];
    let div = arr.len() / 2;
    let slice = &arr[0..div];
    let right_s = &arr[div..];
    let le1 = slice.len();
    let right_s = right_s.len();
    println!("{le1} {right_s}");
}
