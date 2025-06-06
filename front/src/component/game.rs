use super::Board as BoardComp;
use gloo::timers::callback::Interval;
use std::time::Duration;
use tetris::Board as TBoard;
use wasm_timer::Instant;
use yew::{
    function_component, html, use_effect_with, use_force_update, use_mut_ref, use_state, Html,
};

#[function_component]
pub fn Game() -> Html {
    let reactor_sub = yew_agent::reactor::use_reactor_subscription::<crate::component::WsReactor>();
    reactor_sub.send(crate::component::ReactorControlSignal::Start);

    let next_piece_id = use_state(tetris::PieceId::random);
    let next_piece_pos = use_state(|| tetris::Position::from((5, 5)));

    let board = use_mut_ref(|| {
        let mut board = TBoard::default();

        board
            .place_at(
                &tetris::Piece::from(tetris::PieceId::I),
                &tetris::Position::from((4, 19)),
            )
            .unwrap();
        board
            .place_at(
                &tetris::Piece::from(tetris::PieceId::T),
                &tetris::Position::from((2, 17)),
            )
            .unwrap();
        board
            .place_at(
                &tetris::Piece::from(tetris::PieceId::S),
                &tetris::Position::from((4, 17)),
            )
            .unwrap();
        board
            .place_at(
                &tetris::Piece::from(tetris::PieceId::O),
                &tetris::Position::from((0, 18)),
            )
            .unwrap();
        board
            .place_at(
                &tetris::Piece::from(tetris::PieceId::Z),
                &tetris::Position::from((6, 18)),
            )
            .unwrap();
        board
            .place_at(
                &tetris::Piece::from(tetris::PieceId::L),
                &tetris::Position::from((2, 16)),
            )
            .unwrap();
        board
            .place_at(
                &tetris::Piece::from(tetris::PieceId::J),
                &tetris::Position::from((8, 18)),
            )
            .unwrap();
        board
    });
    let last_tick = use_state(Instant::now);
    let fu = use_force_update();

    info!("Game");

    if last_tick.elapsed() > Duration::from_millis(950) {
        let next_pos = (next_piece_pos.x(), next_piece_pos.y() + 1).into();
        let b = board.borrow();
        if b.can_place_at(&tetris::Piece::from(*next_piece_id), &next_pos) {
            next_piece_pos.set(next_pos);
        }else{
            drop(b);
            let mut board = board.borrow_mut();
            if let Err(e) = board.place_at(&tetris::Piece::from(*next_piece_id), &next_piece_pos){
                error!(format!("{e}"))
            }
            next_piece_id.set(tetris::PieceId::random());
            next_piece_pos.set((5, 0).into());
        }
        last_tick.set(Instant::now());
    }

    use_effect_with((), move |_| {
        Interval::new(500, move || {
            fu.force_update();
        })
        .forget();
        || ()
    });

    html! {<>
        <BoardComp board={board} floating_piece={tetris::Piece::from(*next_piece_id)} floating_piece_pos={*next_piece_pos}/>
    </>}
}
