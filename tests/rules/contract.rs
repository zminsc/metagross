use metagross::rules::Position;

#[ignore]
#[test]
fn test_perft_startpos() {
    fn perft<P: Position>(pos: &P, depth: u32) -> u64 {
        let _ = (pos, depth);
        todo!()
    }

    let pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();
    assert_eq!(perft(&pos, 1), 20);
    assert_eq!(perft(&pos, 2), 400);
    assert_eq!(perft(&pos, 3), 8902);
}

#[ignore]
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

#[ignore]
#[test]
fn test_fen_round_trip() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pos = metagross::rules::shakmaty::ShakmatyPosition::from_fen(fen);
    assert!(pos.is_ok(), "Failed to parse starting position FEN");
}

#[ignore]
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

#[ignore]
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
