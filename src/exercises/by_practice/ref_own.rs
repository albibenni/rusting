#![allow(dead_code)]
pub fn ref_own_it() {
    one();
    println!("---");
    two();
}

fn one() {
    let x = 5;
    // Fill the blank
    let p = &x;
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

fn two() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);
    println!("Success Memory address: {:p}, value: {}", y, *y);
}
