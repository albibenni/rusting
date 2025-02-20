#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}
pub fn doit() {
    let mut item = Item { count: 1 };
    println!("{:?}", item);

    add_one(&mut item);
    println!("{:?}", item);
}
