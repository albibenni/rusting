use std::{fmt::Display, str::FromStr};

use super::{
    area::Area,
    collisions::{Contains, Points},
};

#[derive(Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Contains for Circle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        return dx * dx + dy * dy <= self.radius * self.radius;
    }
}

impl Points for Circle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![(self.x, self.y)].into();
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

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("bad circle from str"));
        };
        return Ok(Circle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            radius: parts[2].parse()?,
        });
    }
}
// impl Circle {
//     pub fn contains_point(&self, (x, y): (f64, f64)) -> bool {
//         let dx = self.x - x;
//         let dy = self.y - y;
//         return dx * dx + dy * dy <= self.radius * self.radius;
//     }
// }
// impl Collidable<Rectangle> for Circle{
//     fn collide(&self, other: &Rectangle) -> bool {
//         for point in other {
//             if self.contains_point(point) {
//                 return true;
//             }
//         }
//         return false;
//     }
// }
//
// impl Collidable<Circle> for Circle {
//     fn collide(&self, other: &Circle) -> bool {
//         return self.contains_point((other.x, other.y));
//     }
// }
