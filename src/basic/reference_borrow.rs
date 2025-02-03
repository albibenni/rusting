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
