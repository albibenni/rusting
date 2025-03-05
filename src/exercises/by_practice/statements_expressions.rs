#![allow(dead_code)]

use crate::types::type_of;
#[allow(warnings)]
pub fn using_it() {
    println!("---- STATEMENTS and EXPRESSIONS ----");
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    println!("---- Exercises ----");
    one();
}

#[allow(warnings)]
fn one() {
    let v = {
        let mut x = 1;
        x += 2;
        x
    };

    let sizev = size_of_val(&v);
    println!("v type: {} size: {}", type_of(&v), sizev);

    assert_eq!(v, 3);

    let v = {
        let mut x = 1;
        x += 2
    };

    let sizev = size_of_val(&v);
    println!("v type: {} size: {}", type_of(&v), sizev);

    assert_eq!(v, ());

    let v = {
        let x = 3;
        x
    };

    println!("Success");
}
