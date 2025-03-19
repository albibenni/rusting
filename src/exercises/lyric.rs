#![allow(dead_code)]

pub fn print_lyric() {
    let mut paragrapth = String::new();
    for i in 1..=12 {
        match i {
            1 => {
                paragrapth.push_str("On the first ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(FIRST);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            2 => {
                paragrapth.push_str("On the second ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(SECOND);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            3 => {
                paragrapth.push_str("On the third ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(THIRD);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            4 => {
                paragrapth.push_str("On the forth ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(FORTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            5 => {
                paragrapth.push_str("On the fifth ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(FIFTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            6 => {
                paragrapth.push_str("On the sixth ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(SIXTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            7 => {
                paragrapth.push_str("On the seventh ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(SEVENTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            8 => {
                paragrapth.push_str("On the eighth ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(EIGHTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            9 => {
                paragrapth.push_str("On the ninth ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(NINTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            10 => {
                paragrapth.push_str("On the tenth ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(TENTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            11 => {
                paragrapth.push_str("On the eleventh ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(ELEVENTH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            12 => {
                paragrapth.push_str("On the twelveth ");
                paragrapth.push_str(REPETITION);
                paragrapth.push_str("\n");
                paragrapth.push_str(TWELVETH);
                paragrapth.push_str("\n");
                paragrapth.push_str("\n");
            }
            _ => println!("Invalid number"),
        }
    }
    println!("{}", paragrapth);
}
static REPETITION: &str = "day of Christmas, my true love sent to me";
static FIRST: &str = "A partridge in a pear tree";
static SECOND: &str = "Two turtle doves and
A partridge in a pear tree";
static THIRD: &str = "Three french hens
Two turtle doves and
A partridge in a pear tree";
static FORTH: &str = "Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree";
static FIFTH: &str = "Five golden rings
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
Genius English Translations";
static SIXTH: &str = "Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree";
static SEVENTH: &str = "Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree";
static EIGHTH: &str = "Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree";
static NINTH: &str = "Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree";
static TENTH: &str = "Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves and
A partridge in a pear tree";
static ELEVENTH: &str = "Eleven pipers piping
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
static TWELVETH: &str = "Twelve drummers drumming
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
