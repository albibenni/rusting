#[path = "./basic/shadowing.rs"]
mod shadowing;

#[path = "./prj/guessing_game.rs"]
mod guessing_game;
fn main() {
    print!("Hello I'm rusting");
    //shadowing::shadowing();
    guessing_game::guess();
}
