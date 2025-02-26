mod shapes;
use std::str::FromStr;

use anyhow::{Ok, Result};
use shapes::{
    area::Area,
    circle::Circle,
    collisions::{Collidable, Contains, Points},
    rectangle::Rectangle,
};

#[derive(Debug)]
enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (shape, data) = s.split_once(" ").unwrap_or(("", ""));
        match shape {
            "rect" => return Ok(Shape::Rectangle(data.parse()?)),
            "rectangle" => return Ok(Shape::Rectangle(data.parse()?)),
            "circle" => return Ok(Shape::Circle(data.parse()?)),
            _ => return Err(anyhow::anyhow!("bad shape")),
        }
    }
}

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

    let def_ref = Rectangle::default();
    let def_circle = Circle::default();
    println!("{}", def_ref.area());
    println!("printing the rectangle: {}", def_ref);
    println!("printing the circle: {}", def_circle);

    // for point in def_ref {
    //     println!("print iter rectangle: {:?}", point);
    // }

    let rec1 = Rectangle::default();
    let rec2 = Rectangle::default();
    let circle1 = Circle::default();
    let circle2 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 4.0,
    };

    let coll_r1_r2 = rec1.collide(&rec2);
    let coll_r1_c1 = rec1.collide(&circle1);
    let coll_c1_c2 = rec1.collide(&circle2);
    let coll_c2_r2 = rec1.collide(&rec2);
    println!("Collision r1, r2: {}", coll_r1_r2);
    println!("Collision r1, c1: {}", coll_r1_c1);
    println!("Collision c1, c2: {}", coll_c1_c2);
    println!("Collision c1, r2: {}", coll_c2_r2);
}

impl Points for Shape {
    fn points(&self) -> shapes::collisions::PointIter {
        match self {
            Shape::Rectangle(r) => return r.points(),
            Shape::Circle(c) => return c.points(),
        }
    }
}

impl Contains for Shape {
    fn contains_point(&self, point: (f64, f64)) -> bool {
        match self {
            Shape::Circle(c) => return c.contains_point(point),
            Shape::Rectangle(r) => return r.contains_point(point),
        }
    }
}
pub fn reading_shaped_from_file() -> Result<()> {
    println!("-------- newwww -----");
    let shapes = std::fs::read_to_string("shapes")?
        .lines()
        .filter_map(|x| x.parse::<Shape>().ok())
        .collect::<Vec<_>>();

    println!("test read {:?}", shapes);

    return Ok(());
}
