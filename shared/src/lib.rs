use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    LinesDestroyed (
        // u16 since it's boolean anyway, we can use an int as an array of bits, don't need sign nor more than 16 wide
        Vec<u16>
    ),
    BoardUpdate(tetris::Board),
    GameOver,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Broadcast { user_id: u128, msg: ClientMessage },
    LeaderBoardUpdate(),
}
