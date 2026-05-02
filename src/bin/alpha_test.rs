use chessnt::AnyResult;
use juquad::draw::{draw_rect, to_rect};
use juquad::lazy::add_contour;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

#[macroquad::main("outline")]
async fn main() -> AnyResult<()> {
    let path = "assets/images/characters/peon.png";
    let pawn = macroquad::prelude::load_texture(path).await?;
    pawn.set_filter(FilterMode::Nearest);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        let screen = vec2(screen_width(), screen_height());
        let rect = add_contour(to_rect(vec2(0.0, 0.0), screen), -screen * 0.1);
        draw_rect(rect, WHITE);
        draw_texture(&pawn, screen.x * 0.2, screen.y * 0.2, WHITE);
        next_frame().await
    }

    Ok(())
}
