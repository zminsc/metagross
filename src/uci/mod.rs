pub mod command;
pub mod response;
pub mod session;

use crate::engine::Engine;
use crate::rules::Position;
use std::io::{self, BufRead, Write};

pub fn run<P: Position, E: Engine<P>>(
    engine: E,
    input: impl BufRead,
    mut output: impl Write,
) -> io::Result<()> {
    use command::parse;
    use session::Session;

    let mut session = Session::<P, E>::new(engine);

    for line_result in input.lines() {
        let line = line_result?;

        if let Ok(cmd) = parse(&line) {
            let is_quit = matches!(cmd, command::Command::Quit);
            session.handle(cmd, &mut output)?;
            if is_quit {
                break;
            }
        }
    }

    Ok(())
}
