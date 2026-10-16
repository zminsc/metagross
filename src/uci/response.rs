#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    IdName(String),
    IdAuthor(String),
    UciOk,
    ReadyOk,
    BestMove(String),
}

pub fn format(response: &Response) -> String {
    match response {
        Response::IdName(name) => format!("id name {}", name),
        Response::IdAuthor(author) => format!("id author {}", author),
        Response::UciOk => "uciok".to_string(),
        Response::ReadyOk => "readyok".to_string(),
        Response::BestMove(mv) => format!("bestmove {}", mv),
    }
}
