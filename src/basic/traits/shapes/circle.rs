use std::fmt::Display;

use super::{area::Area, collisions::Collidable, rectangle::Rectangle};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Circle {
    pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl Collidable<Rectangle> for Circle{
    fn collide(&self, other: &Rectangle) -> bool {
        for point in other {
            if self.contains_point(point) {
                return true;
            }
        }
        return false;
    }
}

impl Collidable<Circle> for Circle {
    fn collide(&self, other: &Circle) -> bool {
        return self.contains_point((other.x, other.y));
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * std::f64::consts::PI;
    }
}

impl Default for Circle {
    fn default() -> Self {
        return Circle {
            x: 0.0,
            y: 0.0,
            radius: 10.0,
        };
    }
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "My custom circle, x: {}, y: {}, radius: {}",
            self.x, self.y, self.radius,
        );
    }
}
