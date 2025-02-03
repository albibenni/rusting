pub fn owning_basic() {
    let n = 5;
    let y = plus_one(n);
    println!("Value of y: {y}");
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

fn add_second(mut name: String) -> String {
    name.push_str("benni");
    name
}

pub fn heap_add_suffix() {
    let first = String::from("albi");
    let full = add_second(first);
    // first is moved --
    println!("{full}");
}

pub fn cloning_heap_add_suffix() {
    let first = String::from("albi");
    let first_clone = first.clone();
    let full = add_second(first_clone);
    println!("now it's possible to use the first: {first} and the concatenated: {full}");
}
