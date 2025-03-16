#![allow(dead_code)]
use std::env::Args;

pub struct Fib {
    prev_fib: u64,
    curr_fib: u64,
}
pub fn get_fib(args: &mut Args) -> u64 {
    let args_v: Vec<String> = args.collect();
    let number: u64 = args_v
        .get(1)
        .expect("Eheh nope -- no arg provided")
        .parse()
        .expect("not a number");

    return calc_fib(number);
}

fn calc_fib(numb: u64) -> u64 {
    let mut prev_fib: u64 = 0;
    let mut curr_fib: u64 = 0;
    for i in 0..numb {
        if i == 0 {
            curr_fib = 1;
            continue;
        }
        let temp_new_fib = curr_fib;
        curr_fib += prev_fib;
        prev_fib = temp_new_fib;
    }

    return curr_fib;
}
