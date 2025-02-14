use crate::another_refernce::print_type_of;

pub fn is_option_ye() {
    let foo = Some(6);
    let expl: Option<String> = None;

    print_type_of(&foo);
    print_type_of(&expl);
}
