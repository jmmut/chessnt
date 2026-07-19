use chessnt::core::coord::Coord;
use chessnt::core::input::Gamepads;
use chessnt::core::time::Time;
use chessnt::screen::camera::CameraPos;
use chessnt::screen::shader::init_shaders;
use chessnt::screen::shader::names::{ANTIALIAS_STRENGTH, SCREEN};
use chessnt::screen::theme::{Fonts, Textures, Theme, new_text_coloring};
use chessnt::screen::ui::{SCALE, render_text_no_font, render_title};
use chessnt::screen::ui_dev::DevUi;
use chessnt::world::board::Board;
use chessnt::world::board::board_ui::Message;
use chessnt::world::bot::Bots;
use chessnt::world::moves::Move;
use chessnt::world::team::Team;
use chessnt::{
    AnyResult, DEFAULT_FONT_SIZE, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE,
    DEFAULT_WINDOW_WIDTH, MSAA, PROFILER_ENABLED, Profiler, set_3d_camera,
};
use juquad::widgets::anchor::Anchor;
use macroquad::camera::set_default_camera;
use macroquad::color::{BLACK, Color, WHITE};
use macroquad::input::{
    KeyCode, MouseButton, is_key_down, is_key_pressed, is_mouse_button_down,
    is_mouse_button_pressed, mouse_delta_position,
};
use macroquad::logging::info;
use macroquad::material::{gl_use_default_material, gl_use_material};
use macroquad::math::{Vec2, vec2};
use macroquad::miniquad::FilterMode;
use macroquad::prelude::{
    Conf, DrawTextureParams, RenderTarget, RenderTargetParams, Texture2D, clear_background,
    draw_texture_ex, next_frame, render_target_ex, screen_height, screen_width,
};
use macroquad::prelude::{load_ttf_font, mouse_wheel};
use macroquad::{Error, miniquad};
use std::collections::HashMap;

// const TRANSPARENT_BLACK: Color = Color::new(0.0, 0.0, 0.0, 0.0);
const TRANSPARENT_GREY: Color = Color::new(0.5, 0.5, 0.5, 0.0);

#[macroquad::main(window_conf)]
async fn main() {
    if let Err(e) = fallible_main().await {
        info!("{} failed with error: {}", DEFAULT_WINDOW_TITLE, e);
    }
}

async fn fallible_main() -> AnyResult<()> {
    let mut profiler = Profiler::new(PROFILER_ENABLED);
    let mut screen = vec2(screen_width(), screen_height());
    render_loading().await;
    next_frame().await;

    let mut render_texture = resize(screen);
    let textures = load_textures().await?;
    let fonts = Fonts {
        titles: load_ttf_font("assets/fonts/LilitaOne-Regular.ttf").await?,
        text: load_ttf_font("assets/fonts/TitilliumWeb-SemiBold.ttf").await?,
        dev: load_ttf_font("assets/fonts/JetBrainsMono-Medium.ttf").await?,
    };
    let materials = init_shaders()?;
    let mut theme = Theme::new(textures, fonts, materials);
    let mut camera = CameraPos::default();
    let mut board = Board::new_chess(Coord::new_i(6, 4), Coord::new_i(2, 4));
    let mut bots = Bots::new();
    let mut gamepads = Gamepads::new();
    let mut dev_ui = DevUi::new()?;
    // let mut time = Time::new_fps(Some(55.0));
    let mut time = Time::new_fps(None);
    profiler.end_section("Setup");
    loop {
        let mut frame_profiler = Profiler::new(PROFILER_ENABLED);
        time.tick();
        update_size(&mut screen, &mut render_texture);

        let mut messages = handle_inputs_should_exit(&mut board, &mut gamepads, &mut dev_ui)?;
        gamepads.tick();
        board.tick(time.delta_s());
        bots.tick(time.delta_s(), &mut board)?;
        theme.tick(time.delta_s(), screen)?;

        frame_profiler.end_section("  updates");

        set_3d_camera(&camera, render_texture.clone());
        frame_profiler.end_section("  set camera for 3D");

        clear_background(theme.palette.background);
        board.draw_world(&theme);
        frame_profiler.end_section("  draw 3D world");

        // let mut profiler_2d = Profiler::new(PROFILER_ENABLED);
        set_default_camera();
        frame_profiler.end_section("  camera for 2D");
        // profiler_2d.end_section("2D graphics: set camera");
        clear_background(TRANSPARENT_GREY);
        draw_board_antialias(screen, &render_texture, &theme);
        // profiler_2d.end_section("2D graphics: draw_board");
        messages.extend(board.draw_ui(&theme));
        // profiler_2d.end_section("2D graphics: draw_ui");
        messages.extend(dev_ui.draw(
            &time,
            &mut theme,
            &mut board,
            &mut camera,
            &mut bots,
            &mut gamepads,
        )?);
        // profiler_2d.end_section("2D graphics: dev_ui");

        frame_profiler.end_section("  2D graphics");

        if handle_ui_actions(
            messages,
            &mut time,
            &mut theme,
            &mut board,
            &mut camera,
            &mut bots,
        )
        .await?
        {
            break;
        }

        time.tick_end();
        frame_profiler.end_section("  handle ui actions");
        profiler.end_section("user frame");
        // macroquad_profiler::profiler(Default::default());
        next_frame().await;
        profiler.end_section("macroquad frame");
        profiler.separator();
    }
    Ok(())
}

async fn render_loading() {
    let screen = vec2(screen_width(), screen_height());
    clear_background(BLACK);
    render_text_no_font(
        "Loading...",
        DEFAULT_FONT_SIZE * 2.0,
        new_text_coloring(),
        Anchor::center_v(screen * 0.5),
    );
    next_frame().await;
}

fn update_size(screen: &mut Vec2, render_texture: &mut RenderTarget) {
    let new_screen = vec2(screen_width(), screen_height());
    if new_screen != *screen {
        *screen = new_screen;
        *render_texture = resize(*screen);
    }
}

fn resize(screen: Vec2) -> RenderTarget {
    let render_texture = render_target_ex(
        screen.x as u32,
        screen.y as u32,
        RenderTargetParams {
            sample_count: MSAA,
            depth: false,
        },
    );
    render_texture.texture.set_filter(FilterMode::Linear);
    render_texture
}

async fn load_textures() -> AnyResult<Textures> {
    #[rustfmt::skip]
    let textures = Textures {
        placeholder: load_texture_r("assets/images/ph_chara.png").await?,
        referee: load_texture_r("assets/images/ph_chara.png").await?,
        characters_idle: HashMap::from([
            (Move::Pawn, vec![load_texture_r("assets/images/characters/idle inactive.png").await?]),
            (Move::Rook, vec![load_texture_r("assets/images/characters/torre.png").await?]),
            (Move::Knight, vec![load_texture_r("assets/images/characters/torre.png").await?]), // TODO: replace with correct textures when they exist
            (Move::Bishop, vec![load_texture_r("assets/images/characters/torre.png").await?]), // TODO: replace with correct textures when they exist
            (Move::King, vec![load_texture_r("assets/images/characters/torre.png").await?]), // TODO: replace with correct textures when they exist
            (Move::Queen, vec![load_texture_r("assets/images/characters/peon.png").await?]), // TODO: replace with correct textures when they exist
            // (Move::Knight, load_texture_r("assets/images/characters/caballo.png").await?),
            // (Move::Bishop, load_texture_r("assets/images/characters/alfil.png").await?),
            // (Move::King, load_texture_r("assets/images/characters/rey.png").await?),
            // (Move::Queen, load_texture_r("assets/images/characters/reina.png").await?),
        ]),
    characters_active:  HashMap::from([
            (Move::Pawn, load_textures_r("assets/images/characters/idle active", 2).await?),
            (Move::Rook, vec![load_texture_r("assets/images/characters/torre.png").await?]),
            (Move::Knight, vec![load_texture_r("assets/images/characters/torre.png").await?]), // TODO: replace with correct textures when they exist
            (Move::Bishop, vec![load_texture_r("assets/images/characters/torre.png").await?]), // TODO: replace with correct textures when they exist
            (Move::King, vec![load_texture_r("assets/images/characters/torre.png").await?]), // TODO: replace with correct textures when they exist
            (Move::Queen, vec![load_texture_r("assets/images/characters/peon.png").await?]), // TODO: replace with correct textures when they exist
            // (Move::Knight, load_texture_r("assets/images/characters/caballo.png").await?),
            // (Move::Bishop, load_texture_r("assets/images/characters/alfil.png").await?),
            // (Move::King, load_texture_r("assets/images/characters/rey.png").await?),
            // (Move::Queen, load_texture_r("assets/images/characters/reina.png").await?),
        ]),
        pieces: HashMap::from([
            ((Team::White, Move::Pawn), load_texture_r("assets/images/pieces/icon-w-peon.png").await?),
            ((Team::White, Move::Rook), load_texture_r("assets/images/pieces/icon-w-torre.png").await?),
            ((Team::White, Move::Knight), load_texture_r("assets/images/pieces/icon-w-caballo.png").await?),
            ((Team::White, Move::Bishop), load_texture_r("assets/images/pieces/icon-w-alfil.png").await?),
            ((Team::White, Move::Queen), load_texture_r("assets/images/pieces/icon-w-reina.png").await?),
            ((Team::White, Move::King), load_texture_r("assets/images/pieces/icon-w-rey.png").await?),
            ((Team::Black, Move::Pawn), load_texture_r("assets/images/pieces/icon-b-peon.png").await?),
            ((Team::Black, Move::Rook), load_texture_r("assets/images/pieces/icon-b-torre.png").await?),
            ((Team::Black, Move::Knight), load_texture_r("assets/images/pieces/icon-b-caballo.png").await?),
            ((Team::Black, Move::Bishop), load_texture_r("assets/images/pieces/icon-b-alfil.png").await?),
            ((Team::Black, Move::Queen), load_texture_r("assets/images/pieces/icon-b-reina.png").await?),
            ((Team::Black, Move::King), load_texture_r("assets/images/pieces/icon-b-rey.png").await?),
        ]),
        floor: load_texture_r("assets/images/floor.png").await?,
    };
    Ok(textures)
}
pub async fn load_texture(path: &str) -> Result<Texture2D, Error> {
    let tex = macroquad::prelude::load_texture(path).await?;
    tex.set_filter(FilterMode::Linear);
    Ok(tex)
}
pub async fn load_texture_r(path: &str) -> Result<Texture2D, Error> {
    render_loading().await;
    load_texture(path).await
}
pub async fn load_textures_r(path: &str, count: usize) -> Result<Vec<Texture2D>, Error> {
    let mut textures = Vec::new();
    for i in 1..=count {
        render_loading().await;
        textures.push(load_texture(&format!("{}{}.png", path, i)).await?);
    }
    Ok(textures)
}

fn handle_inputs_should_exit(
    board: &mut Board,
    gamepads: &mut Gamepads,
    dev_ui: &mut DevUi,
) -> AnyResult<Vec<Message>> {
    if is_key_pressed(KeyCode::GraveAccent)
        || is_key_pressed(KeyCode::Slash)
        || is_key_pressed(KeyCode::LeftBracket)
        || is_key_pressed(KeyCode::U)
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
    if is_key_pressed(KeyCode::M) {
        messages.push(Message::ToggleRefreshShaderCharacter);
        messages.push(Message::ToggleRefreshShaderAntialias);
    }
    let wheel = Vec2::from(mouse_wheel());
    if wheel.y > 0.01 {
        messages.push(Message::Zoom(true));
    } else if wheel.y < -0.01 {
        messages.push(Message::Zoom(false));
    }
    let control = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
    let meta = is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper);
    if (control || meta) && is_dragging(MouseButton::Left) {
        messages.push(Message::MoveCamera(Vec2::from(mouse_delta_position())));
    }
    if (control || meta) && is_dragging(MouseButton::Right) {
        messages.push(Message::RotateCamera(Vec2::from(mouse_delta_position())));
    }
    Ok(messages)
}

fn is_dragging(button: MouseButton) -> bool {
    is_mouse_button_down(button) && !is_mouse_button_pressed(button)
}

fn draw_board_antialias(screen: Vec2, render_texture: &RenderTarget, theme: &Theme) {
    if theme.materials.antialias_enabled {
        let material = &theme.materials.antialias;
        gl_use_material(material);
        material.set_uniform(SCREEN, screen);
        material.set_uniform(ANTIALIAS_STRENGTH, theme.materials.antialias_strength);
    }
    draw_texture_ex(
        &render_texture.texture,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(screen),
            flip_y: true,
            ..Default::default()
        },
    );
    if theme.materials.antialias_enabled {
        gl_use_default_material();
    }
}

async fn handle_ui_actions(
    messages: Vec<Message>,
    time: &mut Time,
    theme: &mut Theme,
    board: &mut Board,
    camera: &mut CameraPos,
    bots: &mut Bots,
) -> AnyResult<bool> {
    let _delta_s = time.delta_s();
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
            Message::AnimationFPS(animation_fps) => {
                board.animation_fps = animation_fps;
            }
            Message::TargetFPS(fps) => {
                time.set_target_fps(fps);
            }
            Message::ToggleRefreshShaderCharacter => {
                theme.materials.refresh_shaders.character =
                    !theme.materials.refresh_shaders.character;
            }
            Message::ToggleRefreshShaderAntialias => {
                theme.materials.refresh_shaders.antialias =
                    !theme.materials.refresh_shaders.antialias;
            }
            Message::ToggleShaderAntialias => {
                theme.materials.antialias_enabled = !theme.materials.antialias_enabled;
            }
            Message::ToggleSinCity => {
                theme.materials.sin_city = !theme.materials.sin_city;
            }
            Message::Zoom(increase) => {
                let zoom_speed = 1.2;
                // let zoom_speed = zoom_speed* 60.0 * delta_s as f32;
                if increase {
                    camera.set_zoom_rel(1.0 / zoom_speed);
                } else {
                    camera.set_zoom_rel(zoom_speed);
                }
            }
            Message::MoveCamera(delta) => {
                camera.set_pos_rel(delta);
                camera.set_target_rel(delta);
            }
            Message::RotateCamera(delta) => {
                camera.rotate(delta);
            }
            Message::ShadowOffset(new_value) => {
                theme.materials.shadow_offset = new_value;
            }
            Message::CodeTolerance(new_value) => {
                theme.materials.code_tolerance = new_value;
            }
            Message::AntialiasStrength(new_value) => {
                theme.materials.antialias_strength = new_value;
            }
            Message::FloorAAStrength(new_value) => {
                theme.materials.floor_antialias_strength = new_value;
            }
        }
    }
    Ok(should_exit)
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
        // sample_count: MSAA, // might not be needed if the render target has a >1 sample count?
        platform: miniquad::conf::Platform {
            webgl_version: miniquad::conf::WebGLVersion::WebGL2,
            ..Default::default()
        },
        ..Default::default()
    }
}
