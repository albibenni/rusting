const PATH_TEST_BASE: &str = "src/exercises/";

pub fn error_practice_and_args() {
    let arg = std::env::args()
        .nth(1)
        .expect("provide a file name as argument");

    let mut path = PATH_TEST_BASE.to_owned();
    path.push_str(&arg);

    std::fs::read_to_string(&path)
        .expect("unable to read the file provided")
        .lines()
        .for_each(|line| println!("line: {}", line));

    std::fs::read_to_string(&path)
        .expect("unable to read the file provided")
        .lines()
        .for_each(|line| {
            if let Ok(value) = line.parse::<usize>() {
                println!("{}", value);
            } else {
                println!("Line not a number");
            }
        });
}
