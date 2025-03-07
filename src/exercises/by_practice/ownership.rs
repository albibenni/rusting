pub fn own_it() {
    println!("OWNERSHIP ----");
    one();
    println!("two----");
    two();
    println!("three----");
    three();
    println!("some----");
    use_copy();
    println!("try----");
    tr();
    println!("dereference----");
    dereference();
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

// Don't use clone ,use copy instead
fn use_copy() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

fn tr() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("{}", s1);
}

fn dereference() {
    let x = Box::new(5);
    let mut y = Box::new(1);
    //
    *y = 4;

    assert_eq!(*x, 5);

    println!("Success! {:?}", x);
    assert_ne!(*x, *y);
    println!("Mut with dereference y! {:?}", *y);
}
