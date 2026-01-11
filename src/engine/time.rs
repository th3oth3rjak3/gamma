use std::time::{Duration, Instant};

use crate::gamma::Gamma;

impl<S> Gamma<S> {
    pub fn delta_time(&mut self) -> Duration {
        let current = Instant::now();
        let delta = current - self.last_frame_time;
        self.last_frame_time = current;
        delta
    }
}
