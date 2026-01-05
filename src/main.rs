use juquad::draw::draw_segment;
use juquad::lazy::add_contour;
use macroquad::prelude::*;

pub const COLUMNS: i32 = 8;
pub const ROWS: i32 = 8;

pub struct Coord {
    column: i32,
    row: i32,
}
pub fn coord_to_pixel(coord: Coord, rect: Rect) -> Vec2 {
    let cell_width = rect.w / COLUMNS as f32;
    let cell_height = rect.h / ROWS as f32;
    vec2(cell_width * coord.column as f32, cell_height*coord.row as f32)
}

#[macroquad::main("MY_CRATE_NAME")]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);
        
        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        let screen = Vec2::new(screen_width(), screen_height());
        let rect = add_contour(Rect::new(0.0, 0.0, screen.x, screen.y), -screen * 0.5);
        for column in 0..=COLUMNS {
            let start = coord_to_pixel(Coord{column, row: 0}, rect);
            let end = coord_to_pixel(Coord{column, row: ROWS}, rect);
            draw_segment(start, end, 2.0, DARKGRAY);
        }
        // for column in 0..=COLUMNS {
        //     let start = coord_to_pixel(Coord{column, row: 0}, rect);
        //     let end = coord_to_pixel(Coord{column, row: ROWS}, rect);
        //     draw_segment(start, end, 2.0, DARKGRAY);
        // }
        
        next_frame().await
    }
}
