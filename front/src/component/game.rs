use super::{Board as BoardComp, ReactorControlSignal};
use gloo::timers::callback::Interval;
use shared::ClientMessage;
use std::{cell::RefCell, rc::Rc, time::Duration};
use tetris::Board as TBoard;
use wasm_timer::Instant;
use yew::{
    function_component, html, use_effect_with, use_force_update, use_mut_ref, use_state, Html,
    UseStateHandle,
};
use yew_agent::reactor::UseReactorSubscriptionHandle;

#[function_component]
pub fn Game() -> Html {
    let reactor_sub = yew_agent::reactor::use_reactor_subscription::<crate::component::WsReactor>();
    reactor_sub.send(ReactorControlSignal::Start);

    let next_piece_id = use_state(tetris::PieceId::random);
    let next_piece_pos = use_state(|| tetris::Position::from((5, 5)));

    let board = use_mut_ref(TBoard::default);
    let last_tick = use_state(Instant::now);
    let fu = use_force_update();

    if last_tick.elapsed() > Duration::from_millis(450) {
        game_tick(
            reactor_sub,
            &board,
            next_piece_id.clone(),
            next_piece_pos.clone(),
        );
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

fn game_tick(
    websocket: UseReactorSubscriptionHandle<crate::component::WsReactor>,
    board: &Rc<RefCell<tetris::Board>>,
    next_piece_id: UseStateHandle<tetris::PieceId>,
    next_piece_pos: UseStateHandle<tetris::Position>,
) {
    // debug!("Tick");

    let next_pos = (next_piece_pos.x(), next_piece_pos.y() + 1).into();

    let board_ref = board.borrow();

    if board_ref.can_place_at(&tetris::Piece::from(*next_piece_id), &next_pos) {
        next_piece_pos.set(next_pos);
    } else {
        'place: {
            drop(board_ref);
            let mut board_ref = board.borrow_mut();
            if let Err(e) =
                board_ref.place_at(&tetris::Piece::from(*next_piece_id), &next_piece_pos)
            {
                error!(format!("{e}"));
                break 'place;
            }

            debug!("Board update, sending to reactor");
            websocket.send(ReactorControlSignal::WsMessage(ClientMessage::BoardUpdate(
                board_ref.clone(),
            )));

            next_piece_id.set(tetris::PieceId::random());
            next_piece_pos.set((5, 0).into());
        }
    }
}
