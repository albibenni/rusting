pub fn using_reference() {
    let m1 = String::from("Hello with reference");
    let m2 = String::from("I'm rusting");

    concatenate(&m1, &m2);
    let s = format!("{} {}", m1, m2); // no undefined behavior
    println!("{s}")
}

fn concatenate(p1: &String, p2: &String) -> () {
    println!("{} {}!", p1, p2);
}

pub fn playing_with_it() {
    playing_with_reference_and_dereferencing();
}

fn playing_with_reference_and_dereferencing() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    println!("dereferencing: {a}");
    *x += 1;
    let xx = *x;
    println!("mutated with dereferencing x: {x}");
    println!("but a value is: {a}, didn't mutated");
    println!("a new deference to x has the value mutated: {xx}");

    // reference
    let r1: &Box<i32> = &x;
    let b: i32 = **r1;
    println!("double dereferencing on reference: {b}");

    // reference and dereference
    let r2: &i32 = &*x;
    let c: i32 = *r2;
    println!(
        "r2 points to the heap value directly so only one dereference is needed to read it: {c}"
    );
}
