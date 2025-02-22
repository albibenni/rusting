use std::fmt::Display;

use super::area::Area;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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

pub struct RectangleIter {
    points: Vec<(f64, f64)>,
    idx: usize,
}

impl Iterator for RectangleIter {
    type Item = (f64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        // if self.idx >= self.points.len() {
        //     return None;
        // };
        // let point = self.points[self.idx];
        // self.idx += 1;
        // return Some(point);
        let idx = self.idx;
        self.idx += 1;
        return self.points.get(idx).map(|x| *x);
    }
}

impl IntoIterator for Rectangle {
    type Item = (f64, f64);
    type IntoIter = RectangleIter;

    fn into_iter(self) -> Self::IntoIter {
        return RectangleIter {
            points: vec![
                (self.x, self.y),
                (self.x + self.width, self.y),
                (self.x, self.y + self.height),
                (self.x + self.width, self.y + self.height),
            ],
            idx: 0,
        };
    }
}
