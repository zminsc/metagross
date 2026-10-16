#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Response {
    IdName(String),
    IdAuthor(String),
    UciOk,
    ReadyOk,
    BestMove(String),
}

pub fn format(response: &Response) -> String {
    let _ = response;
    todo!()
}
