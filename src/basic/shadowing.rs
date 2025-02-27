#![allow(dead_code)]
pub fn shadowing() {
    example(5);
}

fn main() {
    example(5);
}

fn example(param: i32) -> () {
    println!("Param before new scope: {}", param);

    {
        let param: String = "Ciao".to_string();

        println!("Param inside new scope: {}", param);
    }

    println!("Param out of new scope: {}", param);
    let param: bool = true;

    println!("Param out of new scope - change type: {}", param);
}
