use gamma::prelude::*;

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
    pub chop: Sound,
    pub out_of_time: Sound,
}

// The update function is called every frame just before the draw function.
pub fn update(gamma: &mut Gamma<GameState>, state: &mut GameState) {
    // We can use the is_key_just_pressed function to see if the key was pressed just before this frame.
    if gamma.is_key_just_pressed(KeyCode::ArrowLeft) {
        state.player_facing = Facing::Left;
        gamma.play_sound(&state.chop);
    }

    if gamma.is_key_just_pressed(KeyCode::ArrowRight) {
        state.player_facing = Facing::Right;
        gamma.play_sound(&state.chop);
    }

    if gamma.is_key_just_released(KeyCode::Space) {
        gamma.play_sound(&state.out_of_time);
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

    let chop = gamma
        .load_sound_from_bytes(include_bytes!("../assets/sounds/chop.wav"))
        .unwrap();

    let mut out_of_time = gamma
        .load_sound_from_bytes(include_bytes!("../assets/sounds/out_of_time.wav"))
        .unwrap();

    out_of_time.set_volume(0.4);

    let state = GameState {
        player_x: 1920.0 / 2.0, // roughly centered
        player_y: 1080.0 / 2.0, // roughly centered
        background,
        player,
        player_facing: Facing::Left,
        chop,
        out_of_time,
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
