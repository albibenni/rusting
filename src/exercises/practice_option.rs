pub fn practice() {
    let ve = Some(5);
    let un = None;
    let p = my_practice(ve);
    let no = my_practice(un);
    println!("Doing some unwrap_or:");

    println!("Undefined index: {}", no);
    println!("Real index: {}", p);
}

fn my_practice(num: Option<isize>) -> isize {
    if let Some(res) = num {
        return res * 5;
    }
    return 0;
}
