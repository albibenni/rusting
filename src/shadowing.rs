fn main() {
    example(5);
}

fn example(param: i32) {
    println!("Param before new scope: {}", param);

    {
        let param: i32 = 35;
        println!("Param inside new scope: {}", param);
    }

    println!("Param out of new scope: {}", param);
}
