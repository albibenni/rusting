pub fn another_one() {
    let just_ez: Vec<_> = vec![1, 2, 3];
    let my_ref = &just_ez[1];
    println!("this is implicit: {my_ref}");
    println!("this is explicit: {}", *my_ref);
    print_type_of(&my_ref);
    print_type_of(&*my_ref);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}
