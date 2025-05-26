use crate::component::Board;
use yew::{function_component, html, Html};

#[derive(yew::Properties, std::cmp::PartialEq)]
pub struct Props {
    pub current_scene: yew::UseStateHandle<crate::scene::Scene>,
}

#[function_component]
pub fn Home(_props: &Props) -> Html {
    if let Some(nav) = yew_router::hooks::use_navigator() {
        nav.replace(&crate::Route::Home)
    } else {
        error!("Failed to retrieve the navigator")
    }

    let reactor_sub = yew_agent::reactor::use_reactor_subscription::<crate::component::WsReactor>();
    reactor_sub.send(crate::component::ReactorControlSignal::Start);

    let mut board = tetris::Board::default();

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
    html! { <>
        <Board  board={board}/>
        // <button onclick={start}>{ "Start ws" }</button>
    </>}
}
