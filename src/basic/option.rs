#![allow(dead_code)]
use crate::another_refernce::print_type_of;

pub fn is_option_ye() {
    let foo = Some(6);
    let expl: Option<String> = None;

    print_type_of(&foo);
    print_type_of(&expl);

    if let Some(x) = foo {
        println!("Lifting Option without unwrap: {}", x);
    }
}

pub fn using_option() {
    let res = ret_a_num_with_undefined(Some(5));
    let res_b = better_ret(Some(15));
    let non: Option<usize> = None;
    let res_und = ret_a_num_with_undefined(non);
    let res_b_und = better_ret(non);
    println!("Option evaluation of some num: {}", res);
    println!("Option evaluation of some none : {}", res_und);
    println!("Option evaluation of some num -- better: {}", res_b);
    println!("Option evaluation of some none -- better : {}", res_b_und);

    let ret_und_too = using_map(non.clone());
    let ret_und_but_no = using_map(Some(3));
    let ret_und_tooo = using_map_and_convert(non.clone());
    let ret_und_but_noo = using_map_and_convert(Some(3));
    println!("Ret undefined - num: {:?}", ret_und_but_no);
    println!("Ret undefined - und: {:?}", ret_und_too);
    println!("Ret undefined with ? - num: {:?}", ret_und_but_noo);
    println!("Ret undefined with ? - und: {:?}", ret_und_tooo);
}

fn ret_a_num_with_undefined(val: Option<usize>) -> usize {
    if let Some(x) = val {
        return x * 5;
    }
    return 0;
}

fn better_ret(val: Option<usize>) -> usize {
    return val.unwrap_or(0) * 5;
}

fn using_map(val: Option<usize>) -> Option<usize> {
    return val.map(|x| x * 5);
}

fn using_map_and_convert(val: Option<usize>) -> Option<usize> {
    // ? does:
    // let num = match val {
    //     Some(x) => x,
    //     None => return None,
    // };
    return Some(val? * 5);
}
