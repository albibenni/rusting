pub fn some_string() {
    let example = "abcd.cdac.com";
    let res = example
        .split(".")
        .collect::<Vec<&str>>()
        .pop()
        .map(|el| el == "com");
    print!("{:?}", res);
}
