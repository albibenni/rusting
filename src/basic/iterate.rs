pub fn iter_myway() {
    let my_v = vec![1, 2, 3, 5, 32, 2];
    for ele in my_v {
        println!("simple for print {}", ele);
    }
}

pub fn another_one() {
    let my_v = vec![1, 2, 3, 5, 32, 2];
    let new_l: Vec<i32> = my_v.iter().map(|el| el + 1).collect();
    new_l
        .iter()
        .for_each(|el| println!("closure with iter and map: {} ", el));
    // cool print
    println!("{:?}", new_l);
}
