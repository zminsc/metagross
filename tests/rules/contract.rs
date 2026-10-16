use metagross::rules::{Position, RulesError};

/// Generic contract tests that any Position implementation must satisfy.
///
/// These tests verify that the implementation correctly:
/// - Computes perft numbers for the starting position
/// - Rejects illegal moves
/// - Round-trips FEN strings correctly

/// Tests perft (performance test) at various depths from the starting position.
///
/// Perft counts the number of leaf nodes at a given depth in the move tree.
/// These are well-known values for the standard starting position:
/// - Depth 1: 20 moves
/// - Depth 2: 400 moves
/// - Depth 3: 8,902 moves
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

/// Tests that illegal moves are properly rejected.
#[ignore]
#[test]
fn test_illegal_move_rejection() {
    let mut pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();

    // Try an illegal move (e.g., moving a piece that doesn't exist)
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

/// Tests that FEN strings can be parsed and serialized correctly.
#[ignore]
#[test]
fn test_fen_round_trip() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let pos = metagross::rules::shakmaty::ShakmatyPosition::from_fen(fen);
    assert!(pos.is_ok(), "Failed to parse starting position FEN");

    // For a complete round-trip test, we would need a to_fen method
    // which is not in the current trait specification
}

/// Tests parsing UCI move strings.
#[ignore]
#[test]
fn test_parse_uci_move() {
    let pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();

    // Test valid move
    let result = pos.parse_uci_move("e2e4");
    assert!(
        result.is_ok(),
        "Failed to parse valid UCI move: {:?}",
        result
    );

    // Test invalid move
    let result = pos.parse_uci_move("e2e5");
    assert!(
        result.is_err(),
        "Expected illegal UCI move to be rejected, but it was accepted"
    );
}

/// Tests converting moves to UCI format.
#[ignore]
#[test]
fn test_to_uci() {
    let pos = metagross::rules::shakmaty::ShakmatyPosition::startpos();

    let m = metagross::rules::Move {
        from: 12, // e2
        to: 28,   // e4
        promotion: None,
    };

    let uci = pos.to_uci(m);
    assert_eq!(uci, "e2e4");
}
