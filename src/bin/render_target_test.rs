use chessnt::{COLUMNS, ROWS};
use macroquad::prelude::*;

#[macroquad::main("Test")]
async fn main() {
    let screen = vec2(screen_width(), screen_height());
    let render_texture = render_target(screen.x as u32, screen.y as u32);
    render_texture.texture.set_filter(FilterMode::Nearest);
    loop {
        // drawing to the texture

        // 0..100, 0..100 camera
        set_camera(&Camera3D {
            up: vec3(0.0, 1.0, 0.0),
            position: vec3(0.0, 0.0, -500.0),
            target: vec3(0.0, 0.0, 0.0),
            render_target: Some(render_texture.clone()),
            ..Default::default()
        });

        clear_background(LIGHTGRAY);
        draw_line(-30.0, 45.0, 30.0, 45.0, 3.0, BLUE);
        draw_circle(-45.0, -35.0, 20.0, YELLOW);
        draw_circle(45.0, -35.0, 20.0, GREEN);

        // drawing to the screen

        set_default_camera();

        clear_background(WHITE);
        draw_texture_ex(
            &render_texture.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
