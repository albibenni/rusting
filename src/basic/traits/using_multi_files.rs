mod shapes;
use shapes::{area::Area, circle::Circle, rectangle::Rectangle};

pub fn doit_multi() {
    let rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let circle = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{}", rect.area());
    println!("{}", circle.area());
    println!("Weird stuff traits on types: {}", 6.9.area());
}
