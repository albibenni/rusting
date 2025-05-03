#![allow(dead_code)]

pub fn some_string() {
    let example = "abcd.cdac.com";
    let res = example
        .split(".")
        .collect::<Vec<&str>>()
        .pop()
        .map(|el| el == "com");
    print!("{:?}", res);
}

pub fn other_string() {
    let site = String::from("ciao.com");

    let example = &site;

    let splitted = example.split(".").collect::<Vec<&str>>();
    let my_slice = &splitted[..splitted.len() - 1];
    println!("splitted, {:?}", my_slice.join("."));
}
