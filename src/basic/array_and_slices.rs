pub fn slicing_around() {
    let mut arr = [2, 3, 4, 5];
    let div = arr.len() / 2;
    let slice = &arr[0..div];
    let right_s = &arr[div..];
    let le1 = slice.len();
    let right_s = slice.len();
    println!("Hello, world {le1} {right_s}");
}
