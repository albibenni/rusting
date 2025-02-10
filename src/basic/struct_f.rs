struct MyStruct {
    x: i32,
    y: i32,
    z: i32,
}

trait Foo {
    fn test(&self) -> ();
}

impl Foo for MyStruct {
    fn test(&self) -> () {
        println!("testing traits and structs with impls --- inside impl after using trait");
    }
}

pub fn testing_struct_tr_imp() {
    let foo = MyStruct {
        x: 50,
        y: 40,
        z: -1,
    };
    let MyStruct { x, .. } = foo;
    println!("destructuring struct {}", x);

    println!("using impl");
    foo.test();

    if let MyStruct { y, z, .. } = foo {
        println!("Pattern matching with struct,{} {} ", y, z)
    }
}

pub fn some_tuple() {
    let tt: (i32, String) = (5, String::from("Tuple"));
    let (n, s) = tt;
    println!("destructuring tuple: {} {}", n, s);
}
