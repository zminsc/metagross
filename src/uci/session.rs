use super::command::Command;
use crate::engine::Engine;
use crate::rules::Position;
use std::io::Write;

/// Maintains UCI session state: current position and engine.
///
/// Handles UCI commands by updating the position, invoking the engine,
/// and writing responses to the output stream.
pub struct Session<P: Position, E: Engine<P>> {
    position: P,
    engine: E,
}

impl<P: Position, E: Engine<P>> Session<P, E> {
    /// Creates a new UCI session with the given engine.
    ///
    /// The initial position is the standard chess starting position.
    pub fn new(engine: E) -> Self {
        Self {
            position: P::startpos(),
            engine,
        }
    }

    /// Handles a single UCI command.
    ///
    /// Updates internal state and writes any necessary responses to `output`.
    ///
    /// # Errors
    ///
    /// Returns `io::Result` if writing to output fails.
    pub fn handle(&mut self, command: Command, output: &mut impl Write) -> std::io::Result<()> {
        let _ = (command, output);
        todo!()
    }
}
