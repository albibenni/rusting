#![allow(dead_code)]
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
