use gamma::prelude::*;

// Create some game state in order to keep track of important state at runtime.
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    pub background: Texture,
    pub player: Texture,
}

// The update function is called every frame just before the draw function.
pub fn update(gamma: &mut Gamma<GameState>, state: &mut GameState) {
    // Let's say the player has a speed of 100 px per second, then we can use delta_time() to move the player
    // delta_time updates the time since the last call, it's very important to call delta_time first thing in
    // the update function and only call it once.
    let delta_time = gamma.delta_time().as_secs_f32();

    state.player_x -= 100.0 * delta_time;
    if state.player_x < 0.0 - state.player.width as f32 {
        state.player_x = 1920.0;
    }
    println!("Player X Position: {}", state.player_x);
}

// The draw function is called every frame after update has finished processing.
pub fn draw(gamma: &mut Gamma<GameState>, state: &mut GameState) {
    gamma.clear_screen(255, 0, 0);
    gamma.draw_texture(&state.background, 0.0, 0.0, Flip::None);
    gamma.draw_texture(&state.player, state.player_x, state.player_y, Flip::None);
}

// The init function is used to create your game state. In more complex scenarios,
// you can use the gamma instance to load textures, audio, fonts, etc. and store them
// in your game state for later reference.
pub fn init(gamma: &mut Gamma<GameState>) -> GameState {
    let background = gamma
        .load_texture_from_bytes(include_bytes!("../assets/graphics/background.png"))
        .unwrap();

    let player = gamma
        .load_texture_from_bytes(include_bytes!("../assets/graphics/player.png"))
        .unwrap();

    let state = GameState {
        player_x: 1920.0,
        player_y: 1080.0 / 2.0, // roughly centered
        background,
        player,
    };

    state
}

pub fn main() {
    let result = GammaBuilder::default()
        // "with" methods are used to change configuration
        .with_title("Timber clone?")
        .with_size(1920, 1080)
        .with_resizable(false)
        .with_vsync(true)
        .with_fullscreen(false)
        // "on" methods are used to give you access to different lifecycle methods
        .on_init(init)
        .on_update(update)
        .on_draw(draw)
        .run();

    if let Err(msg) = result {
        println!("Unexpected error occurred: {msg}");
    }
}
