#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ClientMessage {
    LinesDestroyed (
        // u16 since it's boolean anyway, we can use an int as an array of bits, don't need sign nor more than 16 wide
        Vec<u16>
    ),
    BoardUpdate(tetris::Board),
    GameOver,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum ServerMessage {
    Broadcast { user_id: u128, msg: ClientMessage },
    LeaderBoardUpdate(),
}
