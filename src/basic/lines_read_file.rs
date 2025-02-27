#![allow(dead_code)]
const PATH_LINES: &str = "src/basic/lines.txt";
pub fn read_the_file() {
    println!("Read file ----");

    let file = std::fs::read_to_string(PATH_LINES).unwrap(); // start from root - before
                                                             // src cargo definition
    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));
}

pub fn read_file_with_filters() {
    let file = std::fs::read_to_string(PATH_LINES).unwrap();
    println!("Read with filters ----");

    file.lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}
