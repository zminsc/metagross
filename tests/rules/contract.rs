use metagross::rules::Position;

#[test]
fn test_perft_startpos() {
    fn perft<P: Position>(pos: &P, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = pos.legal_moves();
        if depth == 1 {
            return moves.len() as u64;
        }
        let mut count = 0;
        for m in moves {
            let mut new_pos = pos.clone();
            if new_pos.play(m).is_ok() {
                count += perft(&new_pos, depth - 1);
            }
        }
        count
    }

    let pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();
    assert_eq!(perft(&pos, 1), 20);
    assert_eq!(perft(&pos, 2), 400);
    assert_eq!(perft(&pos, 3), 8902);
}

#[test]
fn test_illegal_move_rejection() {
    let mut pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();

    let illegal_move = metagross::rules::Move {
        from: 0,
        to: 63,
        promotion: None,
    };

    let result = pos.play(illegal_move);
    assert!(
        result.is_err(),
        "Expected illegal move to be rejected, but it was accepted"
    );
}

#[test]
fn test_fen_round_trip() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pos = metagross::rules::shakmaty::ShakmatyPosition::from_fen(fen);
    assert!(pos.is_ok(), "Failed to parse starting position FEN");
}

#[test]
fn test_parse_uci_move() {
    let pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();

    let result = pos.parse_uci_move("e2e4");
    assert!(
        result.is_ok(),
        "Failed to parse valid UCI move: {:?}",
        result
    );

    let result = pos.parse_uci_move("e2e5");
    assert!(
        result.is_err(),
        "Expected illegal UCI move to be rejected, but it was accepted"
    );
}

#[test]
fn test_to_uci() {
    let pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();

    let m = metagross::rules::Move {
        from: 12,
        to: 28,
        promotion: None,
    };

    let uci = pos.to_uci(m);
    assert_eq!(uci, "e2e4");
}

#[test]
fn test_random_self_play() {
    use metagross::engine::{random::RandomEngine, Engine, Limits};

    let mut engine = RandomEngine::new(42);
    let limits = Limits::default();

    for game in 0..1000 {
        let mut pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();
        let mut plies = 0;
        const MAX_PLIES: u32 = 500;

        loop {
            if pos.is_game_over() {
                break;
            }
            if plies >= MAX_PLIES {
                break;
            }

            let m = engine.best_move(&pos, &limits);
            match m {
                None => break,
                Some(m) => {
                    let legal_moves = pos.legal_moves();
                    assert!(
                        legal_moves.contains(&m),
                        "Game {}: Engine returned illegal move at ply {}",
                        game,
                        plies
                    );
                    pos.play(m).expect("Engine returned illegal move");
                    plies += 1;
                }
            }
        }
    }
}

#[test]
fn test_rng_distribution() {
    use metagross::engine::random::RandomEngine;

    let mut rng = RandomEngine::new(12345);
    const CHOICES: usize = 20;
    const DRAWS: usize = 10000;

    let mut counts = [0usize; CHOICES];

    let mut mock_moves = Vec::with_capacity(CHOICES);
    for i in 0..CHOICES {
        mock_moves.push(metagross::rules::Move {
            from: i as u8,
            to: (i + 1) as u8,
            promotion: None,
        });
    }

    for _ in 0..DRAWS {
        let m = rng.bounded(CHOICES as u64) as usize;
        counts[m] += 1;
    }

    let expected = DRAWS / CHOICES;
    let min = *counts.iter().min().unwrap();
    let max = *counts.iter().max().unwrap();

    for (i, &count) in counts.iter().enumerate() {
        assert!(count > 0, "Index {} was never selected", i);
    }

    let tolerance = expected / 3;
    assert!(
        min >= expected.saturating_sub(tolerance),
        "Min count {} too far below expected {}",
        min,
        expected
    );
    assert!(
        max <= expected + tolerance,
        "Max count {} too far above expected {}",
        max,
        expected
    );
}
