#![allow(dead_code)]
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
    println!("eight----");
    eight();
    println!("nine----");
    ninth();
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

// Partial move

fn example() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}

fn eight() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}
fn ninth() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (ref s1, ref s2) = t;

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
