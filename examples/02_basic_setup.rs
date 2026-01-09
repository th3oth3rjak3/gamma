use gamma::prelude::*;

#[derive(Default)]
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_x: 0.0,
            player_y: 0.0,
        }
    }
}

pub fn update(gamma: &mut Gamma<GameState>) {
    // Let's say the player has a speed of 100 px per second, then we can use delta_time() to move the player
    // delta_time updates the time since the last call, it's very important to call delta_time first thing in
    // the update function and only call it once.
    let delta_time = gamma.delta_time().as_secs_f32();
    gamma.state.player_x += 100.0 * delta_time;
    println!("Player X Position: {}", gamma.state.player_x);
}

pub fn draw(gamma: &mut Gamma<GameState>) {
    gamma.clear_screen(255, 0, 0);
}

pub fn main() {
    let state = GameState::new();
    let result = Gamma::new(state)
        .with_title("Red Window")
        .with_size(1920, 1080)
        .with_resizable(false)
        .with_vsync(true)
        .on_update(update)
        .on_draw(draw)
        .run();

    if let Err(msg) = result {
        println!("Unexpected error occurred: {msg}");
    }
}
