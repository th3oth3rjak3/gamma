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

pub fn update(ctx: &mut GammaContext<GameState>) {
    println!("calling update");
}
pub fn draw(ctx: &mut GammaContext<GameState>) {
    println!("calling draw!");
}

pub fn main() {
    let state = GameState::new();
    if let Err(msg) = GammaContext::new(state).draw(draw).update(update).run() {
        println!("Error: {msg}");
    }
}
