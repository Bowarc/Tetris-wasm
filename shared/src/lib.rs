pub enum ClientMessage {
    LineDestroyed {
        // u16 since it's boolean anyway, we can use an int as an array of bits, don't need sign nor more than 16 wide
        line_shapes: Vec<u16>,
        board: tetris::Board,
    },
    BoardUpdate(tetris::Board),
    GameOver,
}

pub enum ServerMessage {
    Broadcast { user_id: i32, msg: ClientMessage },
    LeaderBoardUpdate(),
}
