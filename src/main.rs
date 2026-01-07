use chessnt::coord::Coord;
use chessnt::render::{draw_coord, draw_coord_h};
use chessnt::theme::Theme;
use chessnt::ui::{below_left, render_text};
use chessnt::{
    AnyResult, COLUMNS, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE, DEFAULT_WINDOW_WIDTH,
    FPS_AVERAGE_FRAMES, ROWS,
};
use juquad::widgets::anchor::Anchor;
use macroquad::camera::{set_camera, Camera3D};
use macroquad::color::DARKGREEN;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::{vec2, vec3};
use macroquad::miniquad::date::now;
use macroquad::prelude::{
    clear_background, next_frame, screen_height, screen_width, Conf, LIGHTGRAY,
};
use macroquad::prelude::{load_ttf_font_from_bytes, set_default_camera};

#[macroquad::main(window_conf)]
async fn main() {
    if let Err(e) = fallible_main().await {
        println!("{} failed with error: {}", DEFAULT_WINDOW_TITLE, e);
    }
}

async fn fallible_main() -> AnyResult<()> {
    let mut theme_owned = Theme::default();
    let theme = &mut theme_owned;
    let font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/TitilliumWeb-SemiBold.ttf"))?;
    theme.set_font(font);
    let mut cursor = Coord::new_i(4, 4);
    let mut dev_ui = true;
    let mut last_frame = now();
    let mut frame_count = 0;
    let mut measured_fps = 0.0;
    loop {
        let screen = vec2(screen_width(), screen_height());
        theme.update_screen_size(screen);

        set_camera(&Camera3D {
            position: vec3(0.0, 7., 7.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        if is_key_pressed(KeyCode::Escape) {
            return Ok(());
        }
        if is_key_pressed(KeyCode::Slash) || is_key_pressed(KeyCode::KpDivide) {
            dev_ui = !dev_ui;
        }

        if is_key_pressed(KeyCode::Right) {
            cursor += Coord::new_i(1, 0);
        }
        if is_key_pressed(KeyCode::Left) {
            cursor += Coord::new_i(-1, 0);
        }
        if is_key_pressed(KeyCode::Up) {
            cursor += Coord::new_i(0, -1);
        }
        if is_key_pressed(KeyCode::Down) {
            cursor += Coord::new_i(0, 1);
        }

        clear_background(LIGHTGRAY);

        for column in 0..COLUMNS {
            for row in 0..ROWS {
                let color = if (row + column) % 2 == 0 {
                    theme.palette.white_cells
                } else {
                    theme.palette.black_cells
                };
                draw_coord(Coord::new_i(column, row), color);
            }
        }
        draw_coord_h(cursor, DARKGREEN, 0.3);

        set_default_camera();

        if dev_ui {
            frame_count = (frame_count + 1) % (1000 * FPS_AVERAGE_FRAMES);
            if frame_count % FPS_AVERAGE_FRAMES == 0 {
                let current_frame = now();
                measured_fps = 1.0 / (current_frame - last_frame) * FPS_AVERAGE_FRAMES as f64;
                last_frame = current_frame;
            }
            let rect = render_text("DEV UI", Anchor::top_left(0.0, 0.0), theme);
            let text = "You can move the green cursor with your keyboard arrows";
            let rect = render_text(text, below_left(rect), theme);
            let rect = render_text("Toggle dev UI with '/'", below_left(rect), theme);
            let text = format!("FPS: {:.1}", measured_fps);
            render_text(&text, below_left(rect), theme);
        }

        next_frame().await
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
