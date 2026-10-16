pub mod random;

use crate::rules::{Move, Position};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Limits {
    pub movetime_ms: Option<u64>,
}

pub trait Engine<P: Position> {
    fn best_move(&mut self, pos: &P, limits: &Limits) -> Option<Move>;
}
