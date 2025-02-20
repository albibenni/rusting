#[path = "./shapes.rs"]
mod shapes;

trait Area {
    fn area(&self) -> f64;
}

impl Area for shapes::Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Area for shapes::Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

pub fn doit_multi() {
    let rect = shapes::Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let circle = shapes::Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{}", rect.area());
    println!("{}", circle.area());
}
