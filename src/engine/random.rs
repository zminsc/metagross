use super::{Engine, Limits};
use crate::rules::{Move, Position};

/// A simple engine that plays random legal moves.
pub struct RandomEngine {
    seed: u64,
}

impl RandomEngine {
    /// Creates a new RandomEngine with the given random seed.
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }
}

impl<P: Position> Engine<P> for RandomEngine {
    fn best_move(&mut self, pos: &P, limits: &Limits) -> Option<Move> {
        let _ = (pos, limits);
        todo!()
    }
}
