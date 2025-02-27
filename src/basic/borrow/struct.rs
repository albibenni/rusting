#![allow(dead_code)]

#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}
pub fn doit() {
    let mut item = Item { count: 1 };
    println!("{:?}", item);

    add_one(&mut item);
    println!("{:?}", item);

    let mut itemm = vec![Item { count: 1 }];

    let first = itemm.first_mut();
    println!("{:?}", first);
    print_all(&itemm); // works because after first -- borrowed
}

pub fn re_doit() {
    let mut items = vec![Item { count: 1 }];
    let _first = items.get_mut(0);
    let second = items.get_mut(1);
    //println!("{:?}", first); // error
    println!("{:?}", second); // flow doesnt overlap on error
}
