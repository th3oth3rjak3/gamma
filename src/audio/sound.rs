use std::io::Cursor;

use rodio::{Decoder, Sink};

use crate::gamma::Gamma;

pub struct Sound {
    data: Vec<u8>,
    volume: f32,
}

impl Sound {
    /// Set the relative volume of the sound.
    /// 1.0 is 100% volume, and 0.0 is off.
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
}

impl<S> Gamma<S> {
    pub fn load_sound_from_bytes(&self, bytes: &[u8]) -> Result<Sound, String> {
        let bytes = bytes.to_vec();
        Ok(Sound {
            data: bytes,
            volume: 1.0,
        })
    }

    pub fn play_sound(&mut self, sound: &Sound) {
        let mixer = self.stream_handle.mixer();
        let source = Decoder::new(Cursor::new(sound.data.clone())).expect("failed to decode audio");
        let sink = Sink::connect_new(mixer);
        sink.set_volume(sound.volume);
        sink.append(source);
        sink.detach();
    }
}
