#[path = "./basic/shadowing.rs"]
mod shadowing;

#[path = "./prj/guessing_game.rs"]
mod guessing_game;

#[path = "./basic/functions.rs"]
mod functions;

#[path = "./basic/array_and_slices.rs"]
mod array_and_slices;

#[path = "./basic/ownership.rs"]
mod ownership;

#[path = "./basic/reference_borrow.rs"]
mod reference_borrow;

#[path = "./basic/reference_arrays.rs"]
mod reference_arrays;

#[path = "./basic/referece_dereference_implicit_explicit.rs"]
mod referece_dereference_implicit_explicit;

#[path = "./basic/another_refernce.rs"]
mod another_refernce;

#[path = "./basic/struct_f.rs"]
mod struct_f;

fn main() {
    println!("---- Hello I'm rusting ----");
    //shadowing::shadowing();
    //guessing_game::guess();
    // let res = functions::return_value();
    // println!("return value: {res}");
    // ownership::owning_basic();
    // ownership::heap_add_suffix();
    // ownership::cloning_heap_add_suffix();
    //reference_borrow::using_reference();
    //reference_borrow::playing_with_it();
    //reference_arrays::arr_reference();
    //referece_dereference_implicit_explicit::explicit_implicit_dereference();
    // another_refernce::another_one();
    // another_refernce::trying_boxes();
    struct_f::testing_struct_tr_imp();
    struct_f::some_tuple();
    println!("---- End rusting ----");
}
