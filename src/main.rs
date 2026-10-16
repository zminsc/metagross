use metagross::engine::random::RandomEngine;
use metagross::rules::shakmaty::ShakmatyPosition;
use metagross::uci;
use std::io;

fn main() -> io::Result<()> {
    let engine = RandomEngine::new(0);
    uci::run::<ShakmatyPosition, _>(engine, io::stdin().lock(), io::stdout())
}
