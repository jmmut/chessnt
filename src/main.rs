use chessnt::board::{Board, Move, Team};
use chessnt::coord::Coord;
use chessnt::theme::{CameraPos, Fonts, Textures, Theme};
use chessnt::time::Time;
use chessnt::ui::{DevUi, SCALE};
use chessnt::{
    set_3d_camera, AnyResult, COLUMNS, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE,
    DEFAULT_WINDOW_WIDTH, ROWS,
};
use macroquad::camera::set_default_camera;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{
    clear_background, next_frame, screen_height, screen_width, Conf, LIGHTGRAY,
};
use macroquad::prelude::{load_texture, load_ttf_font};
use std::collections::HashMap;

#[macroquad::main(window_conf)]
async fn main() {
    if let Err(e) = fallible_main().await {
        println!("{} failed with error: {}", DEFAULT_WINDOW_TITLE, e);
    }
}

async fn fallible_main() -> AnyResult<()> {
    #[rustfmt::skip]
    let textures = Textures {
        placeholder: load_texture("assets/images/ph_chara.png").await?,
        pieces: HashMap::from([
            ((Team::White, Move::Pawn), load_texture("assets/images/pieces/icon-w-peon.png").await?),
            ((Team::White, Move::Rook), load_texture("assets/images/pieces/icon-w-torre.png").await?),
            ((Team::White, Move::Knight), load_texture("assets/images/pieces/icon-w-caballo.png").await?),
            ((Team::White, Move::Bishop), load_texture("assets/images/pieces/icon-w-alfil.png").await?),
            ((Team::White, Move::Queen), load_texture("assets/images/pieces/icon-w-reina.png").await?),
            ((Team::White, Move::King), load_texture("assets/images/pieces/icon-w-rey.png").await?),
            ((Team::Black, Move::Pawn), load_texture("assets/images/pieces/icon-b-peon.png").await?),
            ((Team::Black, Move::Rook), load_texture("assets/images/pieces/icon-b-torre.png").await?),
            ((Team::Black, Move::Knight), load_texture("assets/images/pieces/icon-b-caballo.png").await?),
            ((Team::Black, Move::Bishop), load_texture("assets/images/pieces/icon-b-alfil.png").await?),
            ((Team::Black, Move::Queen), load_texture("assets/images/pieces/icon-b-reina.png").await?),
            ((Team::Black, Move::King), load_texture("assets/images/pieces/icon-b-rey.png").await?),
        ]),
    };
    let fonts = Fonts {
        titles: load_ttf_font("assets/fonts/LilitaOne-Regular.ttf").await?,
        text: load_ttf_font("assets/fonts/TitilliumWeb-SemiBold.ttf").await?,
    };
    let mut theme_owned = Theme::new(textures, fonts);
    let theme = &mut theme_owned;
    let mut camera = CameraPos {
        y: 12.69,      // 6.0,
        z: 17.57,      // 8.0,
        fovy: 44.33,   // 45.0,
        target_y: 0.5, // 0.0,
    };
    let mut board = Board::new_chess(
        Coord::new_i(6, 4),
        Coord::new_i(2, 4),
        Coord::new_i(COLUMNS, ROWS),
    );
    let mut dev_ui = DevUi::new();
    let mut time = Time::new();
    loop {
        time.tick();
        let screen = vec2(screen_width(), screen_height());
        theme.update_screen_size(screen);

        if handle_inputs_shoud_exit(&mut board, &mut dev_ui) {
            return Ok(());
        }
        board.tick(time.delta());

        set_3d_camera(&camera);
        clear_background(LIGHTGRAY);
        board.draw_world(theme);

        set_default_camera();
        board.draw_ui(theme);
        dev_ui.draw(&time, theme, &mut board, &mut camera);
        next_frame().await
    }
}

fn handle_inputs_shoud_exit(board: &mut Board, dev_ui: &mut DevUi) -> bool {
    if is_key_pressed(KeyCode::Slash) || is_key_pressed(KeyCode::KpDivide) {
        dev_ui.toggle();
    }

    move_cursor_or_piece(board);

    select(board, &[KeyCode::Space], Team::Black);
    select(board, &[KeyCode::KpEnter, KeyCode::Enter], Team::White);

    if is_key_pressed(KeyCode::KpAdd) {
        unsafe {
            SCALE *= 1.3;
        }
    }
    if is_key_pressed(KeyCode::KpSubtract) {
        unsafe {
            SCALE /= 1.3;
        }
    }
    if is_key_pressed(KeyCode::R) {
        board.reset();
    }
    is_key_pressed(KeyCode::Escape)
}

struct Directions {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
}
fn move_cursor_or_piece(board: &mut Board) {
    const WASD: Directions = Directions {
        up: KeyCode::W,
        down: KeyCode::S,
        left: KeyCode::A,
        right: KeyCode::D,
    };
    const ARROWS: Directions = Directions {
        up: KeyCode::Up,
        down: KeyCode::Down,
        left: KeyCode::Left,
        right: KeyCode::Right,
    };
    move_cursor_or_piece_team(board, Team::White, ARROWS);
    move_cursor_or_piece_team(board, Team::Black, WASD);
}

fn move_cursor_or_piece_team(board: &mut Board, team: Team, directions: Directions) {
    let max = 0.05;
    if board.is_selected(team) {
        let mut delta = Coord::new_f(0.0, 0.0);
        if is_key_down(directions.right) {
            delta += Coord::new_f(0.1, 0.0);
        }
        if is_key_down(directions.left) {
            delta += Coord::new_f(-0.1, 0.0);
        }
        if is_key_down(directions.up) {
            delta += Coord::new_f(0.0, -0.1);
        }
        if is_key_down(directions.down) {
            delta += Coord::new_f(0.0, 0.1);
        }
        if delta != Coord::new_i(0, 0) {
            delta = delta.into::<Vec2>().normalize().into();
            delta *= max;
            board.move_cursor_rel(delta, team);
        }
    } else {
        if is_key_pressed(directions.right) {
            board.move_cursor_rel(Coord::new_i(1, 0), team);
        }
        if is_key_pressed(directions.left) {
            board.move_cursor_rel(Coord::new_i(-1, 0), team);
        }
        if is_key_pressed(directions.up) {
            board.move_cursor_rel(Coord::new_i(0, -1), team);
        }
        if is_key_pressed(directions.down) {
            board.move_cursor_rel(Coord::new_i(0, 1), team);
        }
    }
}

fn select(board: &mut Board, keys: &[KeyCode], team: Team) {
    for key in keys {
        if is_key_pressed(*key) {
            if board.is_selected(team) {
                board.deselect(team);
            } else {
                board.select(team);
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: DEFAULT_WINDOW_TITLE.to_owned(),
        window_width: DEFAULT_WINDOW_WIDTH,
        window_height: DEFAULT_WINDOW_HEIGHT,
        high_dpi: true,
        ..Default::default()
    }
}
