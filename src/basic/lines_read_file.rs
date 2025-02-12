pub fn read_the_file() {
    println!("Read file");

    let file = std::fs::read_to_string("src/basic/lines.txt").unwrap(); // start from root - before
                                                                        // src cargo definition
    file.lines().for_each(|line| println!("{}", line));
}
