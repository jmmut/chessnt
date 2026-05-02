use chessnt::AnyResult;
use juquad::draw::{draw_rect, to_rect};
use juquad::lazy::add_contour;
use macroquad::prelude::*;

#[macroquad::main("outline")]
async fn main() -> AnyResult<()> {
    let path = "assets/images/pieces/icon-w-peon.png";
    let pawn = macroquad::prelude::load_texture(path).await?;
    pawn.set_filter(FilterMode::Nearest);

    let screen = vec2(screen_width(), screen_height());
    let render_texture = render_target_msaa(screen.x as u32, screen.y as u32);
    render_texture.texture.set_filter(FilterMode::Linear);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        set_camera(&Camera2D {
            rotation: 0.0,
            zoom: 2.0 / screen,
            target: screen * 0.5,
            offset: Default::default(),
            render_target: None,
            viewport: None,
        });
        clear_background(BLACK);
        let rect = add_contour(to_rect(vec2(0.0, 0.0), screen), -screen * 0.1);
        draw_rect(rect, WHITE);
        let rect = Rect::new(screen.x * 0.5, rect.y, rect.w * 0.5, rect.h);
        draw_rect(rect, BLACK);
        let pawn_size = pawn.size() * 1.4;
        draw_texture_ex(
            &pawn,
            screen.x * 0.5 - pawn_size.x * 0.5,
            screen.y * 0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(pawn_size),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        next_frame().await
    }

    Ok(())
}
