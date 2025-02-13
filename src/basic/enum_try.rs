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
}


