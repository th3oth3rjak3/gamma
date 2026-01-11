mod builder;
mod engine;
pub mod gamma;
mod rendering;

pub mod prelude {
    pub use crate::builder::GammaBuilder;
    pub use crate::gamma::Gamma;
    pub use crate::rendering::{Flip, Texture};
    pub use winit::keyboard::KeyCode;
}
