use gamma::prelude::*;

fn main() {
    println!("Starting engine example: Opening a window...");
    // We will call a public function from our library to start the engine.
    // This function will contain the winit event loop and all our engine logic.
    let result = Gamma::new(()).on_draw(|_| {}).on_update(|_| {}).run();
    if let Err(msg) = result {
        println!("Had an error: {msg}");
    }
}
