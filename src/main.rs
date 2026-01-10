use chessnt::board::Board;
use chessnt::coord::Coord;
use chessnt::referee::Referee;
use chessnt::theme::{CameraPos, Fonts, Textures, Theme};
use chessnt::ui::{below_left, render_text, rightwards, SCALE};
use chessnt::{
    set_3d_camera, AnyResult, COLUMNS, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE,
    DEFAULT_WINDOW_WIDTH, FPS_AVERAGE_FRAMES, ROWS,
};
use juquad::lazy::{Interactable, Renderable, Style, WidgetTrait};
use juquad::widgets::anchor::Anchor;
use juquad::widgets::Interaction;
use macroquad::camera::set_default_camera;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::{vec2, Rect, Vec2};
use macroquad::miniquad::date::now;
use macroquad::prelude::{
    clear_background, next_frame, screen_height, screen_width, Conf, LIGHTGRAY,
};
use macroquad::prelude::{load_texture, load_ttf_font};

#[macroquad::main(window_conf)]
async fn main() {
    if let Err(e) = fallible_main().await {
        println!("{} failed with error: {}", DEFAULT_WINDOW_TITLE, e);
    }
}

async fn fallible_main() -> AnyResult<()> {
    let textures = Textures {
        placeholder: load_texture("assets/images/ph_chara.png").await?,
    };
    let fonts = Fonts {
        titles: load_ttf_font("assets/fonts/LilitaOne-Regular.ttf").await?,
        text: load_ttf_font("assets/fonts/TitilliumWeb-SemiBold.ttf").await?,
    };
    let mut theme_owned = Theme::new(textures, fonts);
    let theme = &mut theme_owned;
    let mut camera = CameraPos {
        y: 12.69,      //6.0,
        z: 17.57,      // 8.0,
        fovy: 44.33,   // 45.0,
        target_y: 0.5, // 0.0,
    };
    let mut board = Board::new_chess(Coord::new_i(4, 4), Coord::new_i(COLUMNS, ROWS));
    let mut referee = Referee::new();
    let mut dev_ui = true;
    let mut time = Time::new();
    loop {
        time.tick();
        let screen = vec2(screen_width(), screen_height());
        theme.update_screen_size(screen);

        set_3d_camera(&camera);

        if is_key_pressed(KeyCode::Escape) {
            return Ok(());
        }
        if is_key_pressed(KeyCode::Slash) || is_key_pressed(KeyCode::KpDivide) {
            dev_ui = !dev_ui;
        }

        move_cursor_or_piece(&mut board);

        if is_key_pressed(KeyCode::Space) {
            if board.selected() {
                board.deselect();
            } else {
                board.select();
            }
        }

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
        clear_background(LIGHTGRAY);

        board.draw(&camera, theme);

        set_default_camera();
        if dev_ui {
            draw_dev_ui(&time, theme, &mut board, &mut camera);
        }

        next_frame().await
    }
}

pub struct Time {
    pub current_s: f64,
    pub last_s: f64,
    pub frame_count: i32,
    pub rolling_frame_time: f64,
    pub cached_fps: f64,
}

impl Time {
    pub fn new() -> Self {
        let now = now();
        Self {
            current_s: now - 1.0 / 60.0,
            last_s: now - 1.0 / 60.0,
            frame_count: 0,
            rolling_frame_time: 0.0,
            cached_fps: 0.0,
        }
    }
    pub fn tick(&mut self) {
        self.frame_count = (self.frame_count + 1) % (1000 * FPS_AVERAGE_FRAMES);
        self.last_s = self.current_s;
        self.current_s = now();
        self.rolling_frame_time += self.current_s - self.last_s;
        if self.frame_count % FPS_AVERAGE_FRAMES == 0 {
            self.cached_fps = 1.0 / (self.rolling_frame_time / FPS_AVERAGE_FRAMES as f64);
            self.rolling_frame_time = 0.0;
        }
    }
    pub fn fps(&self) -> f64 {
        self.cached_fps
    }
}

fn draw_dev_ui(time: &Time, theme: &mut Theme, board: &mut Board, camera: &mut CameraPos) {
    let _rect = render_text(
        "DEV UI (toggle with '/')",
        Anchor::top_left(0.0, 0.0),
        theme,
    );
    // let text = "You can move the green cursor with your keyboard arrows";
    // let rect = render_text(text, below_left(rect), theme);
    // let rect = render_text("Toggle dev UI with '/'", below_left(rect), theme);
    // let text = format!("FPS: {:.1}", time.fps());
    // let _rect = render_text(&text, below_left(_rect), theme);
    // let text = format!("scale: {}", unsafe { SCALE });
    // let _rect = render_text(&text, below_left(_rect), theme);

    let _rect = render_slider(
        "Texture size X",
        _rect,
        theme,
        &mut board.piece_size.x,
        0.1,
        2.0,
    );
    let _rect = render_slider(
        "Texture size Y",
        _rect,
        theme,
        &mut board.piece_size.y,
        0.1,
        2.0,
    );
    let _rect = render_slider("Camera Y", _rect, theme, &mut camera.y, 0.0, 100.0);
    let _rect = render_slider("Camera Z", _rect, theme, &mut camera.z, 0.0, 100.0);
    let _rect = render_slider("Camera Width", _rect, theme, &mut camera.fovy, 43.5, 47.5);
    let _rect = render_slider(
        "Camera target Y",
        _rect,
        theme,
        &mut camera.target_y,
        -5.0,
        10.0,
    );
}

fn render_slider(
    text: &str,
    rect: Rect,
    theme: &Theme,
    value: &mut f32,
    min: f32,
    max: f32,
) -> Rect {
    let new_rect = render_text(
        &format!("{}: {:0>5.2}", text, value),
        below_left(rect),
        theme,
    );
    let mut slider = juquad::lazy::slider::Slider::new(Style::default(), min, max, *value);
    slider.set_pos(rightwards(new_rect).get_top_left_pixel(slider.size()));
    *value = *(slider
        .interact()
        .into_iter()
        .next()
        .unwrap()
        .downcast::<f32>()
        .unwrap());
    slider.render_interactive(Interaction::None);
    new_rect
}

fn move_cursor_or_piece(board: &mut Board) {
    if board.selected() {
        let mut delta = Coord::new_f(0.0, 0.0);
        let max = 0.05;
        if is_key_down(KeyCode::Right) {
            delta += Coord::new_f(0.1, 0.0);
        }
        if is_key_down(KeyCode::Left) {
            delta += Coord::new_f(-0.1, 0.0);
        }
        if is_key_down(KeyCode::Up) {
            delta += Coord::new_f(0.0, -0.1);
        }
        if is_key_down(KeyCode::Down) {
            delta += Coord::new_f(0.0, 0.1);
        }
        if delta != Coord::new_i(0, 0) {
            delta = delta.into::<Vec2>().normalize().into();
            delta *= max;
            board.move_cursor_rel(delta);
        }
    } else {
        if is_key_pressed(KeyCode::Right) {
            board.move_cursor_rel(Coord::new_i(1, 0));
        }
        if is_key_pressed(KeyCode::Left) {
            board.move_cursor_rel(Coord::new_i(-1, 0));
        }
        if is_key_pressed(KeyCode::Up) {
            board.move_cursor_rel(Coord::new_i(0, -1));
        }
        if is_key_pressed(KeyCode::Down) {
            board.move_cursor_rel(Coord::new_i(0, 1));
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
