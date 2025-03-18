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

static E: &str = "On the first day of Christmas, my true love sent to me
A partridge in a pear tree

On the second day of Christmas, my true love sent to me
Two turtle doves and
A partridge in a pear tree

On the third day of Christmas, my true love sent to me
Three french hens
Two turtle doves and
A partridge in a pear tree

On the fourth day of Christmas, my true love sent to me
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree

On the fifth day of Christmas, my true love sent to me
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree
See pop shows near Milan
Get tickets as low as $101
You might also like
Evil Ways
Drake
OHEMA
Victony
Bad Bunny - MONACO (English Translation)
Genius English Translations

On the sixth day of Christmas, my true love sent to me
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree

On the seventh day of Christmas, my true love sent to me
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree

On the eighth day of Christmas, my true love sent to me
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree

On the ninth day of Christmas, my true love sent to me
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree

On the tenth day of Christmas, my true love sent to me
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree

On the eleventh day of Christmas, my true love sent to me
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree

On the twelfth day of Christmas, my true love sent to me
Twelve drummers drumming
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree";
