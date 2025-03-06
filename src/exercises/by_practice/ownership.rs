pub fn own_it() {
    println!("OWNERSHIP ----");
    one();
}

fn one() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world - reference");
    let y = &x;
    println!("{}, {}", x, y);

    let x = String::from("Hello world - clone");
    let y = x.clone();
    println!("{}, {}", x, y);

    let x = &String::from("Hello world - clone");
    let y = x;
    println!("{}, {}", x, y);
}
