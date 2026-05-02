use chessnt::AnyResult;
use chessnt::screen::ui::{below_left, render_button_dev, render_button_no_font};
use chessnt::screen::ui_dev::on_or_off;
use juquad::draw::{draw_rect, to_rect};
use juquad::lazy::add_contour;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::{Coloring, Interaction};
use macroquad::prelude::*;
use std::iter::Filter;

#[macroquad::main("outline")]
async fn main() -> AnyResult<()> {
    let path = "assets/images/pieces/icon-w-peon.png";
    let pawn = macroquad::prelude::load_texture(path).await?;
    pawn.set_filter(FilterMode::Nearest);

    let screen = vec2(screen_width(), screen_height());
    let render_texture = render_target_msaa(screen.x as u32, screen.y as u32);
    render_texture.texture.set_filter(FilterMode::Nearest);
    let font_size = 16.0;
    let coloring = Coloring::default();

    let mut filter_texture = FilterMode::Nearest;
    let mut filter_target = FilterMode::Nearest;

    let mut use_render_target = false;
    loop {
        pawn.set_filter(filter_texture);
        render_texture.texture.set_filter(filter_target);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if use_render_target {
            set_camera(&Camera2D {
                rotation: 0.0,
                zoom: 2.0 / screen,
                target: screen * 0.5,
                offset: Default::default(),
                render_target: Some(render_texture.clone()),
                viewport: None,
            });
        }
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
        if use_render_target {
            set_default_camera();
            clear_background(WHITE);
            draw_texture_ex(
                &render_texture.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(screen),
                    ..Default::default()
                },
            );
        }
        let mut rect = Rect::default();
        let text = format!("render target: {}", on_or_off(use_render_target));
        if render_button(&text, font_size, &mut rect).is_clicked() {
            use_render_target = !use_render_target;
        }
        let text = format!("filter mode texture: {:?}", filter_texture);
        if render_button(&text, font_size, &mut rect).is_clicked() {
            filter_texture = opposite_filter(filter_texture);
        }
        let text = format!("filter mode render target: {:?}", filter_target);
        if render_button(&text, font_size, &mut rect).is_clicked() {
            filter_target = opposite_filter(filter_target);
        }

        next_frame().await
    }

    Ok(())
}

fn render_button(text: &str, font_size: f32, rect: &mut Rect) -> Interaction {
    const COLORING: Coloring = Coloring::new();
    render_button_no_font(&text, &COLORING, font_size, below_left, rect)
}

fn opposite_filter(filter: FilterMode) -> FilterMode {
    match filter {
        FilterMode::Nearest => FilterMode::Linear,
        FilterMode::Linear => FilterMode::Nearest,
    }
}
