use gloo::{events::EventListener, utils::window};
use js_sys::wasm_bindgen::JsCast as _;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{function_component, html, use_effect, use_state, Html};

#[derive(yew::Properties, PartialEq)]
pub struct Props {
    pub board: tetris::Board,
}

#[function_component]
pub fn Board(props: &Props) -> Html {
    let node_ref = yew::html::NodeRef::default();
    let is_first_rander = use_state(|| true);
    let display_size = use_state(|| (300, 150));
    let size_listen_event_state = use_state(|| EventListener::new(&window(), "resize", |_| ()));

    {
        let node_ref = node_ref.clone();
        let display_size = display_size.clone();
        let board = props.board.clone();

        use_effect(move || {
            if let Some(canvas) = node_ref.cast::<HtmlCanvasElement>() {
                if *is_first_rander {
                    is_first_rander.set(false);
                    let canvas = canvas.clone();

                    display_size.set((canvas.client_width(), canvas.client_height()));

                    size_listen_event_state.set(EventListener::new(
                        &window(),
                        "resize",
                        move |_| {
                            display_size.set((canvas.client_width(), canvas.client_height()));
                        },
                    ));
                }

                render(
                    &canvas,
                    &board,
                    window()
                        .inner_width()
                        .ok()
                        .and_then(|s| s.as_f64())
                        .unwrap()
                        / 4.,
                )
            }

            || ()
        });
    }
    html! { <>
        <canvas ref={node_ref}></canvas>
    </>}
}

fn render(canvas: &HtmlCanvasElement, board: &tetris::Board, w: f64) {
    // Stroke line with should not be odd values, source: https://stackoverflow.com/a/10003573

    let ctx: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let h = w / 0.5;

    canvas.set_width(w as u32);
    canvas.set_height(h as u32);

    ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    draw_grid(&ctx, w, h);

    render_board(&ctx, board, w, h);

    // Borders
    ctx.begin_path();
    ctx.set_stroke_style_str("white");
    ctx.set_line_width(2.);
    ctx.stroke_rect(1., 1., w - 2., h - 2.);
    ctx.close_path();
}

fn render_board(ctx: &CanvasRenderingContext2d, board: &tetris::Board, width: f64, height: f64) {
    let cols = 10;
    let rows = 20;

    let cell_size_x = width / cols as f64;
    let cell_size_y = height / rows as f64;

    for (bit_opt, pos) in board.iter() {
        let Some(bit) = bit_opt else { continue };

        ctx.set_fill_style_str(&bit.color().to_rgba_string());
        ctx.fill_rect(
            pos.x() as f64 * cell_size_x,
            pos.y() as f64 * cell_size_x,
            cell_size_x,
            cell_size_y,
        );
    }
}

fn draw_grid(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    let cols = 10;
    let rows = 20;

    let cell_size_x = width / cols as f64;
    let cell_size_y = height / rows as f64;

    ctx.set_stroke_style_str("lightgrey");
    ctx.set_line_width(2.0);

    for x in (0..(width as usize / cols)).map(|i| i as f64 * cell_size_x) {
        ctx.begin_path();
        ctx.move_to(x, 0.0);
        ctx.line_to(x, height);
        ctx.stroke();
    }

    for y in (0..(height as usize / rows)).map(|i| i as f64 * cell_size_y) {
        ctx.begin_path();
        ctx.move_to(0.0, y);
        ctx.line_to(width, y);
        ctx.stroke();
    }
}

fn draw_piece(
    ctx: &CanvasRenderingContext2d,
    piece: &tetris::Piece,
    position: tetris::Position,
    width: f64,
    height: f64,
) {
    let cols = 10;
    let rows = 20;

    let cell_size_x = width / cols as f64;
    let cell_size_y = height / rows as f64;

    ctx.set_fill_style_str(&piece.color().to_rgba_string());

    for bit in piece.bits().iter() {
        let x = (position.x() as f64 + bit.x() as f64) * cell_size_x;
        let y = (position.y() as f64 + bit.y() as f64) * cell_size_y;
        ctx.fill_rect(x, y, cell_size_x, cell_size_y);
    }

    ctx.set_fill_style_str("black");
    ctx.fill_rect(
        position.x() as f64 * cell_size_x,
        position.y() as f64 * cell_size_y,
        cell_size_x,
        cell_size_y,
    );
}
