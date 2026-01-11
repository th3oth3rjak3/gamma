use gamma::prelude::*;

const PLAYER_X_SPEED: f32 = 500.0;

#[derive(Copy, Clone)]
pub enum Facing {
    Left,
    Right,
}

// Create some game state in order to keep track of important state at runtime.
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    pub background: Texture,
    pub player: Texture,
    pub player_facing: Facing,
}

// The update function is called every frame just before the draw function.
pub fn update(gamma: &mut Gamma<GameState>, state: &mut GameState) {
    let delta_time = gamma.delta_time().as_secs_f32();

    // We can use the is_key_pressed function to see if a user has the key held down.
    if gamma.is_key_pressed(KeyCode::ArrowLeft) {
        state.player_x -= PLAYER_X_SPEED * delta_time;
        state.player_facing = Facing::Left;
    }

    if gamma.is_key_pressed(KeyCode::ArrowRight) {
        state.player_x += PLAYER_X_SPEED * delta_time;
        state.player_facing = Facing::Right;
    }

    println!("Player X Position: {}", state.player_x);
}

// The draw function is called every frame after update has finished processing.
pub fn draw(gamma: &mut Gamma<GameState>, state: &mut GameState) {
    let flip = match state.player_facing {
        Facing::Left => Flip::None,
        Facing::Right => Flip::Horizontal,
    };

    gamma.clear_screen(255, 0, 0);
    gamma.draw_texture(&state.background, 0.0, 0.0, Flip::None);
    gamma.draw_texture(&state.player, state.player_x, state.player_y, flip);
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
        player_x: 1920.0 / 2.0, // roughly centered
        player_y: 1080.0 / 2.0, // roughly centered
        background,
        player,
        player_facing: Facing::Left,
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
        .with_close_on_escape(true)
        // "on" methods are used to give you access to different lifecycle methods
        .on_init(init)
        .on_update(update)
        .on_draw(draw)
        .run();

    if let Err(msg) = result {
        println!("Unexpected error occurred: {msg}");
    }
}
