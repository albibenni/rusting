#![allow(dead_code)]

use std::thread;
use std::time;

#[allow(warnings)]
pub fn do_it() {
    println!("---- Functions ----");
}

// example of thread::sleep
fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    loop {
        println!("I return nothing");
        // sleeping for 1 second to avoid exhausting the cpu resource
        thread::sleep(time::Duration::from_secs(1))
    }
}

fn never_return2() -> ! {
    // implement this function, don't modify fn signatures
    panic!("I return nothing!")
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn()
}

// IMPLEMENT this function
// DON'T change any code else
fn never_return_fn() -> ! {
    unimplemented!()
}
// IMPLEMENT this function in THREE ways
fn never_return_fn2() -> ! {
    panic!()
}
// IMPLEMENT this function in THREE ways
fn never_return_fn3() -> ! {
    todo!();
}
// IMPLEMENT this function in THREE ways
fn never_return_fn4() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
}
