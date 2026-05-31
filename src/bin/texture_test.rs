use chessnt::screen::ui::{render_button_no_font, render_text_no_font_m};
use chessnt::{AnyResult, Profiler};
use juquad::widgets::Coloring;
use juquad::widgets::anchor::Anchor;
use macroquad::prelude::*;

const FONT_SIZE: f32 = 16.0;
const MSAA: i32 = 4;

fn window_conf() -> Conf {
    Conf {
        window_title: "texture test".to_string(),
        high_dpi: true,
        sample_count: MSAA,
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
    let mut profiler_enabled = false;
    loop {
        let mut frame_profiler = Profiler::new(profiler_enabled);
        let mut detail_profiler = Profiler::new(profiler_enabled);
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_down(KeyCode::Space) {
            profiler_enabled = true;
        } else {
            profiler_enabled = false;
        }

        let screen = Rect::new(0.0, 0.0, screen_width(), screen_height());
        let mut rect = Rect::new(screen.w, screen.h, 1.0, 1.0);

        clear_background(Color::new(0.6, 0.5, 0.7, 1.0));
        draw_textures(&pawn);
        draw_ui(&pawn, &coloring, &mut filter_texture, &mut rect);

        detail_profiler.end_section("  user drawing");
        next_frame().await;
        detail_profiler.end_section("  macroquad drawing");
        frame_profiler.end_section("full frame");
        frame_profiler.separator();
    }
    Ok(())
}

fn draw_textures(pawn: &Texture2D) {
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
}

fn draw_ui(
    pawn: &Texture2D,
    coloring: &Coloring,
    filter_texture: &mut FilterMode,
    rect: &mut Rect,
) {
    let toggle_filter = render_button_no_font(
        &format!(
            "currently {:?}, make {:?}",
            filter_texture,
            opposite(*filter_texture)
        ),
        &coloring,
        FONT_SIZE,
        up_left,
        rect,
    );
    if toggle_filter.is_clicked() {
        *filter_texture = opposite(*filter_texture);
        pawn.set_filter(*filter_texture);
    }
    render_text_no_font_m(
        &format!("MSAA: {}", MSAA),
        FONT_SIZE,
        coloring.at_rest,
        up_left,
        rect,
    );
    render_text_no_font_m(
        &format!("FPS: {}", get_fps()),
        FONT_SIZE * 2.0,
        coloring.at_rest,
        up_left,
        rect,
    );
}

fn up_left(rect: Rect) -> Anchor {
    Anchor::bottom_right(rect.x, rect.bottom())
}
// fn left(rect: Rect) -> Anchor {
//     Anchor::top_right(rect.x, rect.y)
// }

fn opposite(filter: FilterMode) -> FilterMode {
    match filter {
        FilterMode::Linear => FilterMode::Nearest,
        FilterMode::Nearest => FilterMode::Linear,
    }
}
