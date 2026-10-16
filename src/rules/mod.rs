pub mod shakmaty;

use std::fmt;

/// Represents a chess move with optional promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    /// Source square (0-63, where 0 is a1, 7 is h1, 56 is a8, 63 is h8).
    pub from: u8,
    /// Destination square (0-63).
    pub to: u8,
    /// Promotion piece, if any ('q', 'r', 'b', 'n').
    pub promotion: Option<char>,
}

/// Errors that can occur when applying chess rules.
#[derive(Debug, Clone)]
pub struct RulesError {
    pub message: String,
}

impl fmt::Display for RulesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RulesError {}

/// A chess position that can generate legal moves and be updated.
///
/// Implementations must maintain a valid chess position and correctly
/// compute legal moves according to chess rules (including en passant,
/// castling, check, checkmate, and stalemate).
pub trait Position: Clone {
    /// Returns the standard chess starting position.
    fn startpos() -> Self;

    /// Parses a position from Forsyth-Edwards Notation.
    ///
    /// # Errors
    ///
    /// Returns `RulesError` if the FEN string is invalid or represents
    /// an illegal position.
    fn from_fen(fen: &str) -> Result<Self, RulesError>;

    /// Returns all legal moves in the current position.
    ///
    /// The returned moves are guaranteed to be legal according to chess rules.
    fn legal_moves(&self) -> Vec<Move>;

    /// Applies a move to this position, mutating it.
    ///
    /// # Errors
    ///
    /// Returns `RulesError` if the move is illegal in the current position.
    fn play(&mut self, m: Move) -> Result<(), RulesError>;

    /// Parses a UCI move string (e.g., "e2e4", "e7e8q") into a Move.
    ///
    /// The move must be legal in the current position.
    ///
    /// # Errors
    ///
    /// Returns `RulesError` if the move string is invalid or the move is illegal.
    fn parse_uci_move(&self, uci: &str) -> Result<Move, RulesError>;

    /// Converts a Move to UCI string format (e.g., "e2e4", "e7e8q").
    fn to_uci(&self, m: Move) -> String;

    /// Returns true if the game is over (checkmate, stalemate, or draw).
    fn is_game_over(&self) -> bool;
}
