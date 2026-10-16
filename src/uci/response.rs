/// A UCI response to send to the GUI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    /// `id name <name>` - Engine name.
    IdName(String),
    /// `id author <author>` - Engine author.
    IdAuthor(String),
    /// `uciok` - UCI handshake complete.
    UciOk,
    /// `readyok` - Engine is ready.
    ReadyOk,
    /// `bestmove <move>` - Best move in UCI notation.
    BestMove(String),
}

/// Formats a UCI response as a string suitable for sending to stdout.
///
/// The returned string does not include a trailing newline.
pub fn format(response: &Response) -> String {
    let _ = response;
    todo!()
}
