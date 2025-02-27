#![allow(dead_code)]
pub fn return_value() -> i16 {
    let y = {
        let x = 3;
        x + 2
    };
    y
}
