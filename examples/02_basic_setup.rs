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
    println!("calling update");
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
        .on_draw(draw)
        .on_update(update)
        .run();

    if let Err(msg) = result {
        println!("Unexpected error occurred: {msg}");
    }
}
