#![allow(dead_code)]
#[derive(Debug)]
struct Custom {
    age: usize,
    name: String,
}

#[derive(Debug)]
enum Item {
    // with type descriminator
    Number(usize),
    String(String),
    MyCustom(Custom),
}

pub fn using_struct_and_enums() {
    let mut items: Vec<Item> = vec![];
    append(&mut items);

    println!("{:?}", items);
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("helloo s".to_string()));
}
