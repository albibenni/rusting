#![allow(dead_code)]
pub fn explicit_implicit_dereference() {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    println!("explicit {x_abs1}, implicit {x_abs2}");

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference twice
    let r_abs2 = r.abs(); // implicit dereference twice
    assert_eq!(r_abs1, r_abs2);
    println!("explicit {r_abs1}, implicit {r_abs2}");

    let s = String::from("Hi");
    let s_let1 = str::len(&s); // explicit reference
    let s_let2 = s.len(); // implicit reference
    assert_eq!(s_let1, s_let2);
    println!("explicit {s_let1}, implicit {s_let2}");
}
