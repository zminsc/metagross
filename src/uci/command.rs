use crate::engine::Limits;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Uci,
    IsReady,
    UciNewGame,
    Position {
        fen: Option<String>,
        moves: Vec<String>,
    },
    Go(Limits),
    Stop,
    Quit,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

pub fn parse(line: &str) -> Result<Command, ParseError> {
    let _ = line;
    todo!()
}
