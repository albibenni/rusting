#[path = "./basic/shadowing.rs"]
mod shadowing;

#[path = "./prj/guessing_game.rs"]
mod guessing_game;

#[path = "./basic/functions.rs"]
mod functions;
fn main() {
    println!("---- Hello I'm rusting ---");
    //shadowing::shadowing();
    //guessing_game::guess();
    let res = functions::return_value();
    println!("return value: {res}");
    println!("---- End rusting ---");
}
