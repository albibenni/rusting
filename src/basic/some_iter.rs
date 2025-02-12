use std::collections::HashMap;

pub fn some_ite() {
    let value: usize = vec![1, 23, 2].iter().sum();
    println!("Sum: {}", value);

    let how_many: usize = vec![1, 23, 1].iter().skip(2).count();
    println!("how_many: {}", how_many);

    vec![1, 2, 5, 9, 4]
        .iter()
        .skip(2)
        .take_while(|&&x| x > 4)
        .for_each(|x| println!("{}", x));

    let what: usize = vec![1, 2, 3].iter().filter(|x| *x % 2 == 0).count();
    println!("what: {}", what);

    let map = HashMap::from([("foo", 1), ("fox", 2), ("bax", 3)]);
    println!("iter for each on HashMap");
    map.iter().for_each(|(k, v)| println!("{}: {}", k, v));
}
