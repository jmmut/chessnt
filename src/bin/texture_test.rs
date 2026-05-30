use chessnt::AnyResult;
use chessnt::screen::ui::render_button_no_font;
use juquad::widgets::Coloring;
use juquad::widgets::anchor::Anchor;
use macroquad::prelude::*;

const FONT_SIZE: f32 = 16.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "texture test".to_string(),
        high_dpi: true,
        sample_count: 13,
        platform: miniquad::conf::Platform {
            webgl_version: miniquad::conf::WebGLVersion::WebGL2,
            ..Default::default()
        },
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() -> AnyResult<()> {
    let path = "assets/images/characters/peon.png";
    let pawn = macroquad::prelude::load_texture(path).await?;
    let mut filter_texture = FilterMode::Linear;
    pawn.set_filter(filter_texture);
    let coloring = Coloring::new();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        let screen = Rect::new(0.0, 0.0, screen_width(), screen_height());
        let mut rect = Rect::new(screen.w, screen.h, 1.0, 1.0);
        clear_background(Color::new(0.6, 0.5, 0.7, 1.0));
        let mut pos = 0.0;
        for scale in [1.0, 0.5, 0.25, 0.2, 0.15, 0.125] {
            draw_texture_ex(
                &pawn,
                pos,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(pawn.size() * scale),
                    ..Default::default()
                },
            );
            pos += scale * pawn.size().x;
        }
        let toggle_filter = render_button_no_font(
            &format!(
                "currently {:?}, make {:?}",
                filter_texture,
                opposite(filter_texture)
            ),
            &coloring,
            FONT_SIZE,
            up_left,
            &mut rect,
        );
        if toggle_filter.is_clicked() {
            filter_texture = opposite(filter_texture);
            pawn.set_filter(filter_texture);
        }
        next_frame().await
    }
    Ok(())
}

fn up_left(rect: Rect) -> Anchor {
    Anchor::bottom_right(rect.x, rect.bottom())
}

fn opposite(filter: FilterMode) -> FilterMode {
    match filter {
        FilterMode::Linear => FilterMode::Nearest,
        FilterMode::Nearest => FilterMode::Linear,
    }
}
