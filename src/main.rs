use chessnt::coord::Coord;
use chessnt::render::{draw_coord, draw_coord_h};
use chessnt::theme::Theme;
use chessnt::ui::render_text;
use chessnt::{
    COLUMNS, DEFAULT_ASPECT_RATIO, DEFAULT_WINDOW_HEIGHT, DEFAULT_WINDOW_TITLE,
    DEFAULT_WINDOW_WIDTH, ROWS,
};
use juquad::lazy::add_contour;
use macroquad::camera::{set_camera, Camera3D};
use macroquad::color::DARKGREEN;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::math::{vec2, vec3, Rect, Vec2};
use macroquad::prelude::set_default_camera;
use macroquad::prelude::{
    clear_background, next_frame, screen_height, screen_width, Conf, LIGHTGRAY,
};

#[macroquad::main(window_conf)]
async fn main() {
    let mut theme_owned = Theme::default();
    let theme = &mut theme_owned;
    let mut cursor = Coord::new_i(4, 4);
    loop {
        let screen = vec2(screen_width(), screen_height());
        theme.update_screen_size(screen);

        clear_background(LIGHTGRAY);

        set_camera(&Camera3D {
            position: vec3(0.0, 7., 7.0),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        // draw_grid(20, 1., BLACK, GRAY);

        if is_key_pressed(KeyCode::Escape) {
            return;
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

        let screen = Vec2::new(screen_width(), screen_height());
        let rect = add_contour(Rect::new(0.0, 0.0, screen.x, screen.y), -screen * 0.25);
        for column in 0..=COLUMNS {
            let start = Coord::new_i(column, 0);
            let end = Coord::new_i(column, ROWS);
            // draw_line_3d(start.into(), end.into(), DARKGRAY);
        }
        for row in 0..=ROWS {
            let start = Coord::new_i(0, row);
            let end = Coord::new_i(COLUMNS, row);
            // draw_line_3d(start.into(), end.into(), DARKGRAY);
        }
        for column in 0..COLUMNS {
            for row in 0..ROWS {
                let color = if (row + column) % 2 == 0 {
                    theme.palette.white_cells
                } else {
                    theme.palette.black_cells
                };

                draw_coord(Coord::new_i(column, row), color);
                // let pixel_00 = coord_to_pixel(coord_00, rect);
                // let pixel_10 = coord_to_pixel(coord_10, rect);
                // let pixel_01 = coord_to_pixel(coord_01, rect);
                // let pixel_11 = coord_to_pixel(coord_11, rect);
                // let color = if (row + column )%2 == 0 { theme.palette.white_cells } else {theme.palette.black_cells};
                // draw_triangle(pixel_00, pixel_10, pixel_01, color);
                // draw_triangle(pixel_01, pixel_10, pixel_11, color);
            }
        }

        draw_coord_h(cursor, DARKGREEN, 0.3);

        set_default_camera();
        render_text(" Move cursor: arrows", 0.0, 0.0, theme);
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
