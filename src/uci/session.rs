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
        use super::response::{format, Response};

        match command {
            Command::Uci => {
                writeln!(
                    output,
                    "{}",
                    format(&Response::IdName("metagross".to_string()))
                )?;
                writeln!(
                    output,
                    "{}",
                    format(&Response::IdAuthor("metagross team".to_string()))
                )?;
                writeln!(output, "{}", format(&Response::UciOk))?;
            }
            Command::IsReady => {
                writeln!(output, "{}", format(&Response::ReadyOk))?;
            }
            Command::UciNewGame => {
                self.position = P::startpos();
            }
            Command::Position { fen, moves } => {
                self.position = if let Some(fen_str) = fen {
                    match P::from_fen(&fen_str) {
                        Ok(pos) => pos,
                        Err(_) => return Ok(()),
                    }
                } else {
                    P::startpos()
                };

                for mv_str in moves {
                    if let Ok(mv) = self.position.parse_uci_move(&mv_str) {
                        let _ = self.position.play(mv);
                    }
                }
            }
            Command::Go(limits) => {
                let best = self.engine.best_move(&self.position, &limits);
                let mv_str = if let Some(mv) = best {
                    self.position.to_uci(mv)
                } else {
                    "0000".to_string()
                };
                writeln!(output, "{}", format(&Response::BestMove(mv_str)))?;
            }
            Command::Stop => {}
            Command::Quit => {}
        }

        Ok(())
    }
}
