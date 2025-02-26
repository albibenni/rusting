use std::{fmt::Display, str::FromStr};

use super::{
    area::Area,
    collisions::{Contains, Points},
};

#[derive(Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Contains for Rectangle {
    fn contains_point(&self, (x, y): (f64, f64)) -> bool {
        return self.x <= x && self.x + self.width >= x && self.y <= y && self.y + self.height >= y;
    }
}

impl Points for Rectangle {
    fn points(&self) -> super::collisions::PointIter {
        return vec![
            (self.x, self.y),
            (self.x + self.width, self.y),
            (self.x, self.y + self.height),
            (self.x + self.width, self.y + self.height),
        ]
        .into();
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "My display Rectangle({}, {}): {}x{}",
            self.x, self.y, self.width, self.height
        );
    }
}

impl FromStr for Rectangle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts.len() != 4 {
            return Err(anyhow::anyhow!("bad rectangle from str"));
        };
        return Ok(Rectangle {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
            width: parts[2].parse()?,
            height: parts[3].parse()?,
        });
    }
}

// -- Generic impl done
// impl Collidable<Rectangle> for Rectangle {
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
// impl Collidable<Circle> for Rectangle{
//     fn collide(&self, other: &Circle) -> bool {
//         return self.contains_point((other.x, other.y));
//     }
// }

// impl IntoIterator for Rectangle {
//     type Item = (f64, f64);
//     type IntoIter = RectangleIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         return RectangleIter {
//             points: vec![
//                 (self.x, self.y),
//                 (self.x + self.width, self.y),
//                 (self.x, self.y + self.height),
//                 (self.x + self.width, self.y + self.height),
//             ],
//             idx: 0,
//         };
//     }
// }

// Generic impl done - not needed any more
// pub struct RectangleIter {
//     points: Vec<(f64, f64)>,
//     idx: usize,
// }
// impl From<&Rectangle> for RectangleIter {
//     fn from(value: &Rectangle) -> Self {
//         return RectangleIter {
//             points: vec![
//                 (value.x, value.y),
//                 (value.x + value.width, value.y),
//                 (value.x, value.y + value.height),
//                 (value.x + value.width, value.y + value.height),
//             ],
//             idx: 0,
//         };
//     }
// }

// impl Iterator for RectangleIter {
//     type Item = (f64, f64);
//
//     fn next(&mut self) -> Option<Self::Item> {
//         // if self.idx >= self.points.len() {
//         //     return None;
//         // };
//         // let point = self.points[self.idx];
//         // self.idx += 1;
//         // return Some(point);
//         let idx = self.idx;
//         self.idx += 1;
//         return self.points.get(idx).map(|x| *x);
//     }
// }
// impl IntoIterator for &Rectangle {
//     type Item = (f64, f64);
//     type IntoIter = RectangleIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         return RectangleIter {
//             points: vec![
//                 (self.x, self.y),
//                 (self.x + self.width, self.y),
//                 (self.x, self.y + self.height),
//                 (self.x + self.width, self.y + self.height),
//             ],
//             idx: 0,
//         };
//     }
// }
// impl IntoIterator for Rectangle {
//     type Item = (f64, f64);
//     type IntoIter = RectangleIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         return (&self).into(); //
//     }
// }
// impl IntoIterator for &Rectangle {
//     type Item = (f64, f64);
//     type IntoIter = RectangleIter;
//
//     fn into_iter(self) -> Self::IntoIter {
//         //return RectangleIter::from(self);
//         return self.into(); //
//     }
// }
