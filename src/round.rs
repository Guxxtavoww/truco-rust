#[derive(Debug, PartialEq)]
pub enum RoundResult {
    Player1,
    Player2,
    Tie,
}

pub fn play_round(round: u32) -> RoundResult {
    // Example: Alternate wins for simplicity (replace with real game rules)
    match round {
        1 => RoundResult::Player1,
        2 => RoundResult::Player2,
        3 => RoundResult::Player1,
        _ => RoundResult::Tie,
    }
}