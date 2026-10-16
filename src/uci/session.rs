use super::command::Command;
use crate::engine::Engine;
use crate::rules::Position;
use std::io::Write;

pub struct Session<P: Position, E: Engine<P>> {
    position: P,
    engine: E,
}

impl<P: Position, E: Engine<P>> Session<P, E> {
    pub fn new(engine: E) -> Self {
        Self {
            position: P::startpos(),
            engine,
        }
    }

    pub fn handle(&mut self, command: Command, output: &mut impl Write) -> std::io::Result<()> {
        let _ = (command, output, &self.position, &mut self.engine);
        todo!()
    }
}
