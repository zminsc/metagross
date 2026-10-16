use metagross::engine::{Engine, Limits};
use metagross::rules::{Move, Position, RulesError};
use metagross::uci::command::{parse, Command};
use metagross::uci::session::Session;

#[derive(Clone)]
struct FakePosition {
    moves_count: usize,
}

impl FakePosition {
    fn new() -> Self {
        Self { moves_count: 0 }
    }
}

impl Position for FakePosition {
    fn startpos() -> Self {
        Self::new()
    }

    fn from_fen(_fen: &str) -> Result<Self, RulesError> {
        Ok(Self::new())
    }

    fn legal_moves(&self) -> Vec<Move> {
        vec![Move {
            from: 12,
            to: 28,
            promotion: None,
        }]
    }

    fn play(&mut self, _m: Move) -> Result<(), RulesError> {
        self.moves_count += 1;
        Ok(())
    }

    fn parse_uci_move(&self, uci: &str) -> Result<Move, RulesError> {
        if uci.len() >= 4 {
            Ok(Move {
                from: 12,
                to: 28,
                promotion: None,
            })
        } else {
            Err(RulesError {
                message: "invalid move".to_string(),
            })
        }
    }

    fn to_uci(&self, _m: Move) -> String {
        "e2e4".to_string()
    }

    fn is_game_over(&self) -> bool {
        false
    }
}

struct FakeEngine;

impl Engine<FakePosition> for FakeEngine {
    fn best_move(&mut self, pos: &FakePosition, _limits: &Limits) -> Option<Move> {
        pos.legal_moves().first().copied()
    }
}

#[test]
fn test_uci_handshake() {
    let mut output = Vec::new();
    let mut session = Session::<FakePosition, FakeEngine>::new(FakeEngine);

    session.handle(Command::Uci, &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("id name metagross"));
    assert!(output_str.contains("id author metagross team"));
    assert!(output_str.contains("uciok"));
}

#[test]
fn test_isready() {
    let mut output = Vec::new();
    let mut session = Session::<FakePosition, FakeEngine>::new(FakeEngine);

    session.handle(Command::Uci, &mut output).unwrap();
    session.handle(Command::IsReady, &mut output).unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("readyok"));
}

#[test]
fn test_position_and_go() {
    let mut output = Vec::new();
    let mut session = Session::<FakePosition, FakeEngine>::new(FakeEngine);

    session.handle(Command::Uci, &mut output).unwrap();
    session.handle(Command::IsReady, &mut output).unwrap();
    session
        .handle(
            Command::Position {
                fen: None,
                moves: vec![],
            },
            &mut output,
        )
        .unwrap();
    session
        .handle(
            Command::Go(Limits {
                movetime_ms: Some(100),
            }),
            &mut output,
        )
        .unwrap();

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("bestmove"));
}

#[test]
fn test_position_with_moves() {
    let mut output = Vec::new();
    let mut session = Session::<FakePosition, FakeEngine>::new(FakeEngine);

    session
        .handle(
            Command::Position {
                fen: None,
                moves: vec!["e2e4".to_string(), "e7e5".to_string()],
            },
            &mut output,
        )
        .unwrap();

    assert!(output.is_empty());
}

#[test]
fn test_position_fen() {
    let mut output = Vec::new();
    let mut session = Session::<FakePosition, FakeEngine>::new(FakeEngine);

    session
        .handle(
            Command::Position {
                fen: Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
                moves: vec![],
            },
            &mut output,
        )
        .unwrap();

    assert!(output.is_empty());
}

#[test]
fn test_parse_uci() {
    let cmd = parse("uci").unwrap();
    assert_eq!(cmd, Command::Uci);
}

#[test]
fn test_parse_isready() {
    let cmd = parse("isready").unwrap();
    assert_eq!(cmd, Command::IsReady);
}

#[test]
fn test_parse_position_startpos() {
    let cmd = parse("position startpos").unwrap();
    assert_eq!(
        cmd,
        Command::Position {
            fen: None,
            moves: vec![],
        }
    );
}

#[test]
fn test_parse_position_startpos_moves() {
    let cmd = parse("position startpos moves e2e4 e7e5").unwrap();
    assert_eq!(
        cmd,
        Command::Position {
            fen: None,
            moves: vec!["e2e4".to_string(), "e7e5".to_string()],
        }
    );
}

#[test]
fn test_parse_position_fen() {
    let cmd =
        parse("position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    assert_eq!(
        cmd,
        Command::Position {
            fen: Some("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()),
            moves: vec![],
        }
    );
}

#[test]
fn test_parse_go_movetime() {
    let cmd = parse("go movetime 1000").unwrap();
    assert_eq!(
        cmd,
        Command::Go(Limits {
            movetime_ms: Some(1000),
        })
    );
}

#[test]
fn test_parse_unknown_command() {
    let result = parse("unknown");
    assert!(result.is_err());
}
