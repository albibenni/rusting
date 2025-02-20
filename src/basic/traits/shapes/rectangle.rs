use crate::multi::Area;

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
