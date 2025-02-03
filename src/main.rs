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
    referece_dereference_implicit_explicit::explicit_implicit_dereference();
    println!("---- End rusting ----");
}
