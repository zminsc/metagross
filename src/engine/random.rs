use super::{Engine, Limits};
use crate::rules::{Move, Position};

pub struct RandomEngine {
    seed: u64,
}

impl RandomEngine {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}

impl<P: Position> Engine<P> for RandomEngine {
    fn best_move(&mut self, pos: &P, limits: &Limits) -> Option<Move> {
        let _ = (pos, limits, self.seed);
        todo!()
    }
}
