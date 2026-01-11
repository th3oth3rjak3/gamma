use std::time::Duration;

use crate::gamma::Gamma;

impl<S> Gamma<S> {
    pub fn delta_time(&mut self) -> Duration {
        self.delta
    }
}
