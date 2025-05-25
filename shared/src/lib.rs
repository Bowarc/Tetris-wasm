pub enum ClientMessage {
    LineDestroyed(Vec<BoardRow>, Board),
    BoardUpdate(Board),
    GameOver,
}

pub enum ServerMessage {
    Broadcast { user_id: i32, msg: ClientMessage },
    LeaderBoardUpdate(),
}

type BoardRow = [i32; 8];

pub struct Board {
    rows: [BoardRow; 16],
}
