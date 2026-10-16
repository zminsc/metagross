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
        self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }

    pub fn bounded(&mut self, bound: u64) -> u64 {
        let mut x = self.next();
        let mut m = (x as u128).wrapping_mul(bound as u128);
        let mut l = m as u64;
        if l < bound {
            let t = bound.wrapping_neg() % bound;
            while l < t {
                x = self.next();
                m = (x as u128).wrapping_mul(bound as u128);
                l = m as u64;
            }
        }
        (m >> 64) as u64
    }
}

impl<P: Position> Engine<P> for RandomEngine {
    fn best_move(&mut self, pos: &P, limits: &Limits) -> Option<Move> {
        let _ = limits;
        let moves = pos.legal_moves();
        if moves.is_empty() {
            return None;
        }
        let index = self.bounded(moves.len() as u64) as usize;
        Some(moves[index])
    }
}
