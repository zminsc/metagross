use super::{Move, Position, RulesError};

#[derive(Clone)]
pub struct ShakmatyPosition {
    _inner: (),
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
