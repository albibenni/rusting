#![allow(dead_code)]
use std::{env::Args, fmt::Display};

pub struct CelFah {
    celsius: f64,
    farhrenheit: f64,
}

impl Display for CelFah {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "Celsius: {}, Farhrenheit: {}",
            self.celsius, self.farhrenheit
        );
    }
}

pub fn cel_to_fah_and_back(args: &mut Args) -> CelFah {
    let args_v: Vec<String> = args.collect();
    let conversion = args_v.get(1).expect("Eheh nope -- no conversion type");
    let number: f64 = args_v
        .get(2)
        .expect("Eheh nope -- number not present")
        .parse()
        .expect("Not a num");
    let result = match conversion.to_lowercase().as_str() {
        "c" => CelFah {
            celsius: number,
            farhrenheit: number * 9.0 / 5.0 + 32.0,
        },
        "f" => CelFah {
            celsius: number * 5.0 / 9.0 - 32.0,
            farhrenheit: number,
        },

        _ => CelFah {
            celsius: 0.0,
            farhrenheit: 0.0,
        },
    };
    return result;
}
