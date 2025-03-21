use std::env::args;

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

#[path = "./basic/iterate.rs"]
mod iterate;

#[path = "./basic/custom_collect.rs"]
mod custom_collect;

#[path = "./basic/some_iter.rs"]
mod some_iter;

#[path = "./basic/lines_read_file.rs"]
mod lines_read_file;

#[path = "./basic/enum_try.rs"]
mod enum_try;

#[path = "./basic/enum_sol.rs"]
mod enum_sol;

#[path = "./basic/option.rs"]
mod option;

#[path = "./exercises/practice_pattern_matching.rs"]
mod practice_pattern_matching;

#[path = "./exercises/practice_option.rs"]
mod practice_option;

#[path = "./exercises/error_handling.rs"]
mod error_handling;

#[path = "./basic/borrow/struct.rs"]
mod struct_r;

#[path = "./basic/traits/example.rs"]
mod example;

#[path = "./basic/traits/using_multi_files.rs"]
mod multi;

#[path = "./exercises/by_practice/variables.rs"]
mod variables;

#[path = "./exercises/by_practice/types.rs"]
mod types;

#[path = "./exercises/by_practice/functions.rs"]
mod functions_f;
#[path = "./exercises/by_practice/statements_expressions.rs"]
mod statements_expressions;

#[path = "./exercises/by_practice/ownership.rs"]
mod ownership_ex;
#[path = "./exercises/workshop/p1.rs"]
mod p1;
// mod basic {
//     pub mod super_nested;
// }

mod basic {
    pub mod super_nested {
        pub mod more_nested;
    }
}

mod exercises;

fn main() {
    println!("---- Hello I'm rusting ----");
    let mut _args = args();
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
    // struct_f::testing_struct_tr_imp();
    // struct_f::some_tuple();
    // iterate::iter_myway();
    // iterate::another_one();
    // custom_collect::coll();
    // some_iter::some_ite();
    // lines_read_file::read_the_file();
    // lines_read_file::read_file_with_filters();
    //enum_try::use_enum();
    //enum_sol::using_struct_and_enums();
    // option::is_option_ye();
    // option::using_option();
    //practice_pattern_matching::practice();
    //practice_option::practice();
    //error_handling::error_practice_and_args();
    // struct_r::doit();
    // struct_r::re_doit();
    //example::doit();
    // multi::doit_multi();
    // let res = multi::reading_shaped_from_file();
    // println!("res of reading is: {:?}", res);

    //variables::variab();
    //types::using_types();
    //statements_expressions::using_it();
    //functions_f::do_it();
    // ownership_ex::own_it();
    //super_nested::test_nested();
    //super_nested::more_nested::example();
    //p1::println_population();

    //let res = exercises::celsius_to_fahrenheit::cel_to_fah_and_back(&mut args);
    //let res = exercises::nth_fib::get_fib(&mut args);
    //println!("{}", res);
    //exercises::lyric::print_lyric();
    //exercises::by_practice::ownership::own_it();
    //exercises::by_practice::ref_own::ref_own_it();
    exercises::slicing::slice();
    println!("---- End rusting ----");
}
