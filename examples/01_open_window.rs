use gamma::prelude::*;

fn draw(gamma: &mut Gamma<()>, _state: &mut ()) {
    // At a minimum we need to call clear_screen in order to show a window
    gamma.clear_screen(255, 255, 255);
}

fn main() {
    // Use the gamma builder to create a new gamma instance and then call run on it to start the game engine.
    let result = GammaBuilder::default().on_draw(draw).run();

    // Don't forget to handle errors!
    if let Err(msg) = result {
        println!("Had an error: {msg}");
    }
}
