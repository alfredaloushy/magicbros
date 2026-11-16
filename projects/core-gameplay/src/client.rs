//! Shared frame-to-tick input handling for presentation adapters.
use crate::model::PlayerCommand;

#[derive(Default)]
pub struct BufferedInput {
    pub command: PlayerCommand,
    pub held_cast: bool,
    pending_cast: bool,
}

impl BufferedInput {
    /// Preserve a press even when it is released before the next simulation tick.
    pub fn press_cast(&mut self) {
        self.pending_cast = true;
    }

    pub fn take_tick(&mut self) -> PlayerCommand {
        let mut command = self.command;
        command.cast = self.held_cast || self.pending_cast;
        self.command.clear_edges();
        self.pending_cast = false;
        command
    }
}

#[derive(Default)]
pub struct FixedClock {
    accumulator: f64,
}

impl FixedClock {
    /// Cap stalls to 100ms. Paused frames must reset the clock instead of advancing.
    pub fn advance(&mut self, seconds: f64, tick: f64) -> usize {
        if !seconds.is_finite() || seconds < 0.0 || !tick.is_finite() || tick <= 0.0 {
            return 0;
        }
        self.accumulator += seconds.min(0.1);
        // Bound even pathological tick values; never loop billions of times.
        let ticks = (self.accumulator / tick).floor().min(240.0) as usize;
        self.accumulator %= tick;
        ticks
    }
}
