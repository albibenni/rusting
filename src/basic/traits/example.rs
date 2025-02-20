struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

pub fn doit() {
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
}
