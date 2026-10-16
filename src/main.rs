use metagross::engine::random::RandomEngine;
use metagross::rules::shakmaty::ShakmatyPosition;
use metagross::uci;
use std::io;
use std::time::SystemTime;

fn main() -> io::Result<()> {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let engine = RandomEngine::new(seed);
    uci::run::<ShakmatyPosition, _>(engine, io::stdin().lock(), io::stdout())
}
