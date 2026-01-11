use gamma::prelude::*;

// Create some game state in order to keep track of important state at runtime.
// We need to derive default to allow you to use the init function to load textures with the engine.
#[derive(Default)]
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
}

// The update function is called every frame just before the draw function.
pub fn update(gamma: &mut Gamma<GameState>, state: &mut GameState) {
    // Let's say the player has a speed of 100 px per second, then we can use delta_time() to move the player
    let delta_time = gamma.delta_time().as_secs_f32();

    state.player_x += 100.0 * delta_time;
    if state.player_x > 1920.0 {
        state.player_x = -500.0;
    }

    println!("Player X Position: {}", state.player_x);
}

// The draw function is called every frame after update has finished processing.
pub fn draw(gamma: &mut Gamma<GameState>, _state: &mut GameState) {
    // Call clear screen at a minimum in order for the window to show.
    gamma.clear_screen(255, 0, 0);
}

// The init function is used to create your game state. In more complex scenarios,
// you can use the gamma instance to load textures, audio, fonts, etc. and store them
// in your game state for later reference.
pub fn init(_gamma: &mut Gamma<GameState>) -> GameState {
    let state = GameState {
        player_x: -500.0,
        player_y: 0.0,
    };

    state
}

pub fn main() {
    let result = GammaBuilder::default()
        // "with" methods are used to change configuration
        .with_title("Red Window")
        .with_size(1920, 1080)
        .with_resizable(false)
        .with_vsync(true)
        // "on" methods are used to give you access to different lifecycle methods
        .on_init(init)
        .on_update(update)
        .on_draw(draw)
        .run();

    if let Err(msg) = result {
        println!("Unexpected error occurred: {msg}");
    }
}
