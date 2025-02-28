#![allow(dead_code)]

pub fn variab() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32 = 1; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");

    let x = define_x();
    println!("{}, world", x);

    dest();
}

fn define_x() -> &'static str {
    return "hello";
}

fn dest() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    println!("{},{}", x, y);
    assert_eq!([x, y], [3, 2]);

    println!("Success!");
}
