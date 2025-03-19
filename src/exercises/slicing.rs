use crate::types::type_of;

pub fn slice() {
    let start_string = String::from("some string are cool");
    let res = first_word(&start_string);
    println!("hellooo slice index:  {res}");
    let slices = start_string.split_at(res);
    println!("Slice: {}", slices.0);
    let start_string = String::from("somestringarecool");
    let res = first_word(&start_string);
    println!("hellooo slice with no spaces, index:  {res}");
    let slices = start_string.split_at(res);
    println!("Slice: {}", slices.0);

    // -- better way of slicing
    let start_string = String::from("some strings are cooler");
    let res = first_word(&start_string);
    println!("hellooo slice clooler, index:  {res}");
    let slice = &start_string[0..res];
    let rest = &start_string[res..start_string.len()];
    println!("Slice: {}", &slice);
    println!("Rest of Slice: {}", &rest);
    println!("Type of: {}", type_of(&slice));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{i}, item: {item}");
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}
