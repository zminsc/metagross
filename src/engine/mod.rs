pub mod random;

use crate::rules::{Move, Position};

/// Time and depth constraints for move search.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Limits {
    /// Maximum time to spend on this move, in milliseconds.
    pub movetime_ms: Option<u64>,
}

/// A chess engine that selects moves for a given position.
///
/// Engines receive a position and search limits, and must return their
/// chosen move. Engines know nothing about UCI protocol details.
pub trait Engine<P: Position> {
    /// Chooses the best move for the given position within the given limits.
    ///
    /// Returns `None` if the position is terminal (checkmate, stalemate, or draw)
    /// and no move is legal.
    fn best_move(&mut self, pos: &P, limits: &Limits) -> Option<Move>;
}
