#![allow(dead_code)]
pub fn coll() {
    let data = vec![1, 3, 5];
    let mut foo = data.iter().map(|x| x + 1);

    let mut new_vec = vec![];

    while let Some(x) = foo.next() {
        new_vec.push(x);
    }

    println!("{:?}", new_vec)
}
