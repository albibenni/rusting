pub fn variab() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32 = 1; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}
