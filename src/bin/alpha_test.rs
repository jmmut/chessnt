use chessnt::AnyResult;
use chessnt::screen::ui::{below_left, render_button_no_font};
use chessnt::screen::ui_dev::on_or_off;
use juquad::draw::{draw_rect, to_rect};
use juquad::lazy::add_contour;
use juquad::widgets::{Coloring, Interaction};
use macroquad::prelude::*;

#[macroquad::main("outline")]
async fn main() -> AnyResult<()> {
    let path = "assets/images/pieces/icon-w-peon.png";
    let pawn = macroquad::prelude::load_texture(path).await?;
    pawn.set_filter(FilterMode::Nearest);

    let screen = vec2(screen_width(), screen_height());
    let render_texture = render_target_msaa(screen.x as u32, screen.y as u32);
    render_texture.texture.set_filter(FilterMode::Nearest);
    let font_size = 16.0;

    let mut filter_texture = FilterMode::Linear;
    let mut filter_target = FilterMode::Nearest;

    let mut clear_default_camera = WHITE;
    let mut clear_render_target = WHITE;

    let mut use_render_target = true;
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
            clear_background(clear_render_target);
        } else {
            clear_background(clear_default_camera);
        }
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
            clear_background(clear_default_camera);
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

        let text = format!("clear default camera: {:?}", clear_default_camera);
        if render_button(&text, font_size, &mut rect).is_clicked() {
            clear_default_camera = opposite_color(clear_default_camera);
        }
        let text = format!("clear render_target: {:?}", clear_render_target);
        if render_button(&text, font_size, &mut rect).is_clicked() {
            clear_render_target = opposite_color(clear_render_target);
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

fn opposite_color(color: Color) -> Color {
    let black = color.r < 0.5;
    let transparent = color.a < 0.5;
    if transparent {
        if black {
            WHITE
        } else {
            Color::new(0.0, 0.0, 0.0, 0.0)
        }
    } else {
        if black {
            Color::new(1.0, 1.0, 1.0, 0.0)
        } else {
            BLACK
        }
    }
}
