#![allow(dead_code)]

pub fn practice() {
    let ve = vec![1, 2, 3];
    let p = real_practice(&ve, 2);
    let no = real_practice(&ve, 10);
    println!("Doing some unwrap_or:");

    println!("Undefined index: {}", no);
    println!("Real index: {}", p);
}

fn real_practice(nums: &Vec<usize>, index: usize) -> usize {
    let r = nums.get(index);
    let res = r.unwrap_or(&index) * 5;
    return res;
}
