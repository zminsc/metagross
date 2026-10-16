use crate::engine::Limits;

/// A UCI command from the GUI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    /// `uci` - Tell engine to use UCI protocol.
    Uci,
    /// `isready` - Ping the engine.
    IsReady,
    /// `ucinewgame` - Start a new game.
    UciNewGame,
    /// `position [fen <fen> | startpos] moves <move1> ... <moveN>`
    Position {
        /// FEN string if provided, None for startpos.
        fen: Option<String>,
        /// List of moves in UCI notation to apply after the position.
        moves: Vec<String>,
    },
    /// `go` with time controls and limits.
    Go(Limits),
    /// `stop` - Stop searching and return best move immediately.
    Stop,
    /// `quit` - Exit the engine.
    Quit,
}

/// Errors that can occur when parsing UCI commands.
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

/// Parses a single UCI command from a line of text.
///
/// # Errors
///
/// Returns `ParseError` if the command is malformed or unknown.
pub fn parse(line: &str) -> Result<Command, ParseError> {
    let _ = line;
    todo!()
}
