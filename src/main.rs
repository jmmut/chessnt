use chessnt::core::coord::Coord;
use chessnt::core::input::Gamepads;
use chessnt::core::time::Time;
use chessnt::screen::shader::init_shaders;
use chessnt::screen::theme::{CameraPos, Fonts, Textures, Theme, new_text_coloring};
use chessnt::screen::ui::{SCALE, render_text_no_font, render_title};
use chessnt::screen::ui_dev::DevUi;
use chessnt::world::board::Board;
use chessnt::world::board::board_ui::Message;
use chessnt::world::bot::Bots;
use chessnt::world::moves::Move;
use chessnt::world::team::Team;
use chessnt::{
    AnyResult, DEFAULT_FONT_SIZE, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE,
    DEFAULT_WINDOW_WIDTH, set_3d_camera,
};
use juquad::widgets::anchor::Anchor;
use macroquad::camera::set_default_camera;
use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use macroquad::math::vec2;
use macroquad::prelude::{Conf, clear_background, next_frame, screen_height, screen_width};
use macroquad::prelude::{load_texture, load_ttf_font};
use std::collections::HashMap;

#[macroquad::main(window_conf)]
async fn main() {
    if let Err(e) = fallible_main().await {
        println!("{} failed with error: {}", DEFAULT_WINDOW_TITLE, e);
    }
}

async fn fallible_main() -> AnyResult<()> {
    let screen = vec2(screen_width(), screen_height());
    render_text_no_font(
        "Loading...",
        DEFAULT_FONT_SIZE * 2.0,
        new_text_coloring(),
        Anchor::center_v(screen * 0.5),
    );
    next_frame().await;
    let textures = load_textures().await?;
    let fonts = Fonts {
        titles: load_ttf_font("assets/fonts/LilitaOne-Regular.ttf").await?,
        text: load_ttf_font("assets/fonts/TitilliumWeb-SemiBold.ttf").await?,
        dev: load_ttf_font("assets/fonts/JetBrainsMono-Medium.ttf").await?,
    };
    let material = init_shaders()?;
    let mut theme = Theme::new(textures, fonts, material);
    let mut camera = CameraPos::default();
    let mut board = Board::new_chess(Coord::new_i(6, 4), Coord::new_i(2, 4));
    let mut bots = Bots::new();
    let mut gamepads = Gamepads::new();
    let mut dev_ui = DevUi::new()?;
    let mut time = Time::new();
    loop {
        time.tick();
        gamepads.tick();
        let screen = vec2(screen_width(), screen_height());
        theme.update_screen_size(screen);

        let mut messages = handle_inputs_shoud_exit(&mut board, &mut gamepads, &mut dev_ui)?;
        board.tick(time.delta());
        bots.tick(time.delta(), &mut board)?;

        set_3d_camera(&camera);
        clear_background(theme.palette.background);
        board.draw_world(&theme);

        set_default_camera();
        messages.extend(board.draw_ui(&theme));
        messages.extend(dev_ui.draw(
            &time,
            &mut theme,
            &mut board,
            &mut camera,
            &mut bots,
            &mut gamepads,
        )?);
        if handle_ui_actions(messages, &mut board, &mut bots, &mut time, &mut theme).await? {
            break;
        }
        time.tick_end();
        next_frame().await
    }
    Ok(())
}

async fn handle_ui_actions(
    messages: Vec<Message>,
    board: &mut Board,
    bots: &mut Bots,
    time: &mut Time,
    theme: &mut Theme,
) -> AnyResult<bool> {
    let mut should_exit = false;
    for message in messages {
        match message {
            Message::Exit => {
                should_exit = true;
            }
            Message::Restart => {
                board.reset();
                bots.restart();
            }
            Message::ReloadTextures => {
                let anchor = Anchor::center_v(theme.screen_rect().center());
                render_title("Re-loading textures", theme, anchor);
                next_frame().await;
                theme.textures = load_textures().await?;
            }
            Message::ToggleBot(team) => {
                bots.bots.get_mut(team).toggle();
            }

            Message::ToggleRadar => {
                board.referee.render_radar = !board.referee.render_radar;
            }
            Message::ToggleReferee => {
                board.referee.referee_paused = !board.referee.referee_paused;
            }
            Message::TargetFPS(fps) => {
                time.set_target_fps(fps);
            }
        }
    }
    Ok(should_exit)
}

async fn load_textures() -> AnyResult<Textures> {
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
    Ok(textures)
}

fn handle_inputs_shoud_exit(
    board: &mut Board,
    gamepads: &mut Gamepads,
    dev_ui: &mut DevUi,
) -> AnyResult<Vec<Message>> {
    if is_key_pressed(KeyCode::Slash)
        || is_key_pressed(KeyCode::KpDivide)
        || is_key_pressed(KeyCode::LeftBracket)
    {
        dev_ui.toggle();
    }

    move_cursor_or_piece(board, gamepads)?;

    select(board, &[KeyCode::Space], Team::Black)?;
    select(board, &[KeyCode::KpEnter, KeyCode::Enter], Team::White)?;

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
    let mut messages = Vec::new();
    if is_key_pressed(KeyCode::R) {
        messages.push(Message::Restart)
    }
    if is_key_pressed(KeyCode::P) {
        messages.push(Message::ToggleReferee)
    }
    if is_key_pressed(KeyCode::O) {
        messages.push(Message::ToggleRadar)
    }
    if is_key_pressed(KeyCode::Escape) {
        messages.push(Message::Exit);
    }
    if is_key_pressed(KeyCode::T) {
        messages.push(Message::ReloadTextures);
    }
    if is_key_pressed(KeyCode::B) {
        messages.push(Message::ToggleBot(Team::Black));
    }
    if is_key_pressed(KeyCode::V) {
        messages.push(Message::ToggleBot(Team::White));
    }
    Ok(messages)
}

struct Directions {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
}
fn move_cursor_or_piece(board: &mut Board, gamepads: &mut Gamepads) -> AnyResult<()> {
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

    // println!("before gamepads");
    gamepads.move_cursor_or_piece(board)
    // println!("after gamepads");
}

fn move_cursor_or_piece_team(board: &mut Board, team: Team, directions: Directions) {
    let max = 0.05; // TODO: make dependent on frame delay?
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
            delta = delta.normalize();
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

fn select(board: &mut Board, keys: &[KeyCode], team: Team) -> AnyResult<()> {
    for key in keys {
        if is_key_pressed(*key) {
            board.toggle_select(team)?;
        }
    }
    Ok(())
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
