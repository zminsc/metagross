use crate::engine::Limits;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Uci,
    IsReady,
    UciNewGame,
    Position {
        fen: Option<String>,
        moves: Vec<String>,
    },
    Go(Limits),
    Stop,
    Quit,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

pub fn parse(line: &str) -> Result<Command, ParseError> {
    let tokens: Vec<&str> = line.split_whitespace().collect();

    if tokens.is_empty() {
        return Err(ParseError {
            message: "empty command".to_string(),
        });
    }

    match tokens[0] {
        "uci" => Ok(Command::Uci),
        "isready" => Ok(Command::IsReady),
        "ucinewgame" => Ok(Command::UciNewGame),
        "stop" => Ok(Command::Stop),
        "quit" => Ok(Command::Quit),
        "position" => parse_position(&tokens[1..]),
        "go" => parse_go(&tokens[1..]),
        _ => Err(ParseError {
            message: format!("unknown command: {}", tokens[0]),
        }),
    }
}

fn parse_position(tokens: &[&str]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError {
            message: "position requires argument".to_string(),
        });
    }

    let (fen, rest) = match tokens[0] {
        "startpos" => (None, &tokens[1..]),
        "fen" => {
            let mut fen_end = 1;
            while fen_end < tokens.len() && tokens[fen_end] != "moves" {
                fen_end += 1;
            }
            if fen_end == 1 {
                return Err(ParseError {
                    message: "fen requires argument".to_string(),
                });
            }
            let fen_str = tokens[1..fen_end].join(" ");
            (Some(fen_str), &tokens[fen_end..])
        }
        _ => {
            return Err(ParseError {
                message: format!("invalid position type: {}", tokens[0]),
            })
        }
    };

    let moves = if !rest.is_empty() && rest[0] == "moves" {
        rest[1..].iter().map(|s| s.to_string()).collect()
    } else if rest.is_empty() {
        Vec::new()
    } else {
        return Err(ParseError {
            message: format!("unexpected token after position: {}", rest[0]),
        });
    };

    Ok(Command::Position { fen, moves })
}

fn parse_go(tokens: &[&str]) -> Result<Command, ParseError> {
    let mut limits = Limits::default();

    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "movetime" {
            i += 1;
            if i >= tokens.len() {
                return Err(ParseError {
                    message: "movetime requires value".to_string(),
                });
            }
            limits.movetime_ms = Some(tokens[i].parse().map_err(|_| ParseError {
                message: format!("invalid movetime value: {}", tokens[i]),
            })?);
        }
        i += 1;
    }

    Ok(Command::Go(limits))
}
