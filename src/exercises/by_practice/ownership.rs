pub fn own_it() {
    println!("OWNERSHIP ----");
    one();
    println!("two----");
    two();
    println!("three----");
    three();
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

fn two() {
    // Don't modify code in main!
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    return s;
}

fn three() {
    let s = give_ownership();
    println!("{:?}", s);
}

// Only modify the code below!
//fn give_ownership() -> &'static Vec<u8> {
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.clone().into_bytes();
    let _s2 = s.as_bytes();
    println!("into_bytes: {:?}", _s);
    println!("as_bytes: {:?}", _s2);
    return s;
}
