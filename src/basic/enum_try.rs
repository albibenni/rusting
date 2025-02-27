#![allow(dead_code)]
use self::Color::*;
use std::slice::Iter;

#[derive(Debug)]
enum Color {
    Blue,
    Green,
    Yellow,
    Red,
}

impl Color {
    pub fn iterator() -> Iter<'static, Color> {
        static COLORS: [Color; 4] = [Blue, Green, Yellow, Red];
        COLORS.iter()
    }
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
    fn is_green_parts(&self) -> bool {
        match self {
            Blue => return true,
            Green => return false,
            Yellow => return true,
            Red => return false,
        }
    }
}
pub fn use_enum() {
    for color in Color::iterator() {
        match color {
            Color::Blue => println!("blue"),
            Color::Green => println!("Green"),
            Color::Yellow => println!("Yellow"),
            Color::Red => println!("red"),
        };
    }

    let foo_green = Color::Green.is_green();
    let foo_blue = Color::Green.is_green_parts();
    println!("using impl is green, {}", foo_green);
    println!("using impl is green part, {}", foo_blue);
}
