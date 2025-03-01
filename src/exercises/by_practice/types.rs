#![allow(dead_code)]
pub fn using_types() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");

    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success!, {}, equal to {}", "i32".to_string(), type_of(&x));
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
