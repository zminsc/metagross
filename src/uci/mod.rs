pub mod command;
pub mod response;
pub mod session;

use crate::engine::Engine;
use crate::rules::Position;
use std::io::{self, BufRead, Write};

/// Runs the UCI protocol loop.
///
/// Reads UCI commands from `input`, maintains position and engine state,
/// and writes UCI responses to `output`. Continues until the `quit` command
/// is received.
///
/// # Errors
///
/// Returns `io::Result` if reading from input or writing to output fails.
pub fn run<P: Position, E: Engine<P>>(
    engine: E,
    input: impl BufRead,
    output: impl Write,
) -> io::Result<()> {
    let _ = (engine, input, output);
    todo!()
}
