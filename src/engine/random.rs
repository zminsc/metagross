use super::{Engine, Limits};
use crate::rules::{Move, Position};

pub struct RandomEngine {
    state: u64,
}

impl RandomEngine {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }
}

impl<P: Position> Engine<P> for RandomEngine {
    fn best_move(&mut self, pos: &P, limits: &Limits) -> Option<Move> {
        let _ = limits;
        let moves = pos.legal_moves();
        if moves.is_empty() {
            return None;
        }
        let index = (self.next() as usize) % moves.len();
        Some(moves[index])
    }
}
