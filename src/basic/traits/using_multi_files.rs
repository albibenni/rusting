mod shapes;
use shapes::{area::Area, circle::Circle, collisions::Collidable, rectangle::Rectangle};

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
