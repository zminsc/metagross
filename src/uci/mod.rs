pub mod command;
pub mod response;
pub mod session;

use crate::engine::Engine;
use crate::rules::Position;
use std::io::{self, BufRead, Write};

pub fn run<P: Position, E: Engine<P>>(
    engine: E,
    input: impl BufRead,
    output: impl Write,
) -> io::Result<()> {
    let _ = (engine, input, output);
    todo!()
}
