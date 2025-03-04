#![allow(dead_code)]

use std::ops::{Range, RangeInclusive};
pub fn using_types() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!, {}, equal to {}", "i32".to_string(), type_of(&x));

    println!("____");
    float_eq();
    conversion();
    range_incl();

    bitwise();
    char_size();

    let c1 = '中';
    print_char(&c1);
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

/*

nice for testing floating point equality
*/
fn float_eq() {
    let a: f32 = 0.1;
    let b: f32 = 0.2;
    let tot: f32 = 0.3;
    let res = (a + b - tot).abs();
    assert!(res < 0.01);
    //assert!(a + b == 0.30, "a = {}, b = {}", a, b);
    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
    assert!(0.1f32 + 0.2f32 == 0.3f32);
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 0.001);

    let a = 3;
    let b = 27;
    assert!(a + b == 30, "a = {}, b = {}", a, b);

    println!("Success!");
}

fn conversion() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u16);
    }
}

fn range_incl() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("assert_eq!((1..=5), RangeInclusive::new(1, 5))");
    println!("Success!");
}

fn bitwise() {
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

fn char_size() {
    let c1 = 'a';
    let size1 = size_of_val(&c1);
    println!("c1 type: {} size: {}", type_of(&c1), size1);
    assert_eq!(size_of_val(&c1), 4);
    let c2 = '中';
    let size2 = size_of_val(&c2);
    println!("c2 type: {} size: {}", type_of(&c2), size2);
    assert_eq!(size_of_val(&c2), 4);
    let s1 = "a";
    let size3 = size_of_val(&s1);
    println!("s1 type: {} size: {}", type_of(&s1), size3);
    assert_eq!(size_of_val(&s1), 16);
    let unit: () = ();
    let size4 = size_of_val(&unit);
    println!("unit type: {} size: {}", type_of(&unit), size4);
    assert_eq!(size_of_val(&unit), 0);
}

fn print_char(c: &char) {
    println!("{}", c);
}
