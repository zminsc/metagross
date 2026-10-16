use super::{Move, Position, RulesError};

/// Chess position implementation backed by the shakmaty library.
///
/// This is the only module allowed to import shakmaty. It adapts shakmaty's
/// types to our `Position` trait interface.
#[derive(Clone)]
pub struct ShakmatyPosition {
    _inner: (), // Placeholder; real implementation would hold shakmaty types
}

impl Position for ShakmatyPosition {
    fn startpos() -> Self {
        todo!()
    }

    fn from_fen(fen: &str) -> Result<Self, RulesError> {
        let _ = fen;
        todo!()
    }

    fn legal_moves(&self) -> Vec<Move> {
        todo!()
    }

    fn play(&mut self, m: Move) -> Result<(), RulesError> {
        let _ = m;
        todo!()
    }

    fn parse_uci_move(&self, uci: &str) -> Result<Move, RulesError> {
        let _ = uci;
        todo!()
    }

    fn to_uci(&self, m: Move) -> String {
        let _ = m;
        todo!()
    }

    fn is_game_over(&self) -> bool {
        todo!()
    }
}
