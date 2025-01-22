#[path = "./basic/shadowing.rs"]
mod shadowing;
fn main() {
    print!("Hello I'm rusting");
    shadowing::shadowing();
}
