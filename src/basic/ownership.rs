pub fn owning_basic() {
    let n = 5;
    let y = plus_one(n);
    println!("Value of y: {y}");
}

fn plus_one(n: i32) -> i32 {
    n + 1
}

pub fn heap_add_suffix() {
    let first = String::from("albi");
    let full = add_second(first);
    println!("{full}");
}

fn add_second(mut name: String) -> String {
    name.push_str("benni");
    name
}
