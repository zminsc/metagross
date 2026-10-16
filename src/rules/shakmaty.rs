use super::{Move, Position, RulesError};
use shakmaty::{Chess, Position as ShakmatyTrait};

#[derive(Clone)]
pub struct ShakmatyPosition {
    inner: Chess,
}

impl Position for ShakmatyPosition {
    fn startpos() -> Self {
        Self {
            inner: Chess::default(),
        }
    }

    fn from_fen(fen: &str) -> Result<Self, RulesError> {
        let fen: shakmaty::fen::Fen = fen.parse().map_err(|e| RulesError {
            message: format!("Invalid FEN: {}", e),
        })?;
        let inner = fen
            .into_position(shakmaty::CastlingMode::Standard)
            .map_err(|e| RulesError {
                message: format!("Invalid position: {}", e),
            })?;
        Ok(Self { inner })
    }

    fn legal_moves(&self) -> Vec<Move> {
        self.inner
            .legal_moves()
            .into_iter()
            .map(|m| Move {
                from: m.from().map(|sq| sq.into()).unwrap_or(0),
                to: m.to().into(),
                promotion: m.promotion().map(|p| match p {
                    shakmaty::Role::Queen => 'q',
                    shakmaty::Role::Rook => 'r',
                    shakmaty::Role::Bishop => 'b',
                    shakmaty::Role::Knight => 'n',
                    _ => 'q',
                }),
            })
            .collect()
    }

    fn play(&mut self, m: Move) -> Result<(), RulesError> {
        let legal = self.inner.legal_moves();
        for candidate in legal {
            let candidate_from = candidate.from().map(|sq| sq.into()).unwrap_or(0);
            let candidate_to: u8 = candidate.to().into();
            let candidate_promotion = candidate.promotion().map(|p| match p {
                shakmaty::Role::Queen => 'q',
                shakmaty::Role::Rook => 'r',
                shakmaty::Role::Bishop => 'b',
                shakmaty::Role::Knight => 'n',
                _ => 'q',
            });

            if candidate_from == m.from
                && candidate_to == m.to
                && candidate_promotion == m.promotion
            {
                self.inner.play_unchecked(&candidate);
                return Ok(());
            }
        }

        Err(RulesError {
            message: "Illegal move".to_string(),
        })
    }

    fn parse_uci_move(&self, uci: &str) -> Result<Move, RulesError> {
        let m: shakmaty::uci::UciMove = uci.parse().map_err(|e| RulesError {
            message: format!("Invalid UCI: {}", e),
        })?;
        let shakmaty_move = m.to_move(&self.inner).map_err(|e| RulesError {
            message: format!("Invalid move: {}", e),
        })?;

        if !self.inner.is_legal(&shakmaty_move) {
            return Err(RulesError {
                message: "Illegal move".to_string(),
            });
        }

        Ok(Move {
            from: shakmaty_move.from().map(|sq| sq.into()).unwrap_or(0),
            to: shakmaty_move.to().into(),
            promotion: shakmaty_move.promotion().map(|p| match p {
                shakmaty::Role::Queen => 'q',
                shakmaty::Role::Rook => 'r',
                shakmaty::Role::Bishop => 'b',
                shakmaty::Role::Knight => 'n',
                _ => 'q',
            }),
        })
    }

    fn to_uci(&self, m: Move) -> String {
        let from = shakmaty::Square::new(m.from.into());
        let to = shakmaty::Square::new(m.to.into());
        let promotion = m.promotion.and_then(|p| match p {
            'q' => Some(shakmaty::Role::Queen),
            'r' => Some(shakmaty::Role::Rook),
            'b' => Some(shakmaty::Role::Bishop),
            'n' => Some(shakmaty::Role::Knight),
            _ => None,
        });

        let uci = shakmaty::uci::UciMove::Normal {
            from,
            to,
            promotion,
        };
        uci.to_string()
    }

    fn is_game_over(&self) -> bool {
        self.inner.is_game_over()
    }
}
