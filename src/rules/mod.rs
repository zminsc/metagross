pub mod shakmaty;

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<char>,
}

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

pub trait Position: Clone {
    fn startpos() -> Self;
    fn from_fen(fen: &str) -> Result<Self, RulesError>;
    fn legal_moves(&self) -> Vec<Move>;
    fn play(&mut self, m: Move) -> Result<(), RulesError>;
    fn parse_uci_move(&self, uci: &str) -> Result<Move, RulesError>;
    fn to_uci(&self, m: Move) -> String;
    fn is_game_over(&self) -> bool;
}
