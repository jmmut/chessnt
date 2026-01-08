use crate::theme::Theme;
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::text::TextRect;
use juquad::widgets::{StateStyle, Widget};
use macroquad::math::Rect;
use macroquad::prelude::{Font, TextParams};

pub fn render_text(text: &str, anchor: Anchor, theme: &Theme) -> Rect {
    let t = TextRect::new_generic(
        text,
        anchor,
        theme.font_size(),
        Some(theme.font()),
        macroquad::prelude::measure_text,
    );
    draw_rect(t.rect(), theme.coloring().at_rest.bg_color);
    t.render_default(&theme.coloring().at_rest);
    t.rect()
}
pub static mut SCALE: f32 = 13.78;
/*
pub fn render_text_3d(text: &str, anchor: Anchor, z: f32, theme: &Theme) -> Mesh {
    panic!("doesn't work! this creates a memory leak");
    push_camera_state();

    let t = TextRect::new_generic(
        text,
        anchor,
        theme.font_size(),
        Some(theme.font()),
        macroquad::prelude::measure_text,
    );

    let test_text = "FFFF";
    let mut m = measure_text(test_text, None, theme.font_size() as u16, 1.0);
    m.width = 20.0 * unsafe { SCALE };
    m.height = 10.0 * unsafe { SCALE };
    // let render_target = macroquad::prelude::render_target(m.width as u32, m.height as u32);
    let render_target = macroquad::prelude::render_target(
        (t.rect().w * unsafe { SCALE }) as u32,
        (t.rect().h * unsafe { SCALE }) as u32,
    );
    // render_target.texture.set_filter(FilterMode::Nearest);
    set_camera(&Camera2D {
        //     target: vec2(sw * 0.5, sh * 0.5),
        // target: vec2(m.width, m.height),
        // target: vec2(0.0, 0.0),
        target: t.rect().size(),
        zoom: vec2(1.0, 1.0) / t.rect().size() / unsafe { SCALE } * 0.005,
        // zoom: vec2(1.0 / m.width, 1.0 / m.height) / Vec2::splat(unsafe { SCALE } * 0.005),
        // zoom: vec2(1.0 / m.width, 1.0/ m.height) / Vec2::splat(0.76) * 2.0,
        render_target: Some(render_target),
        ..Default::default()
    });
    clear_background(GRAY);
    // draw_rect(t.rect(), theme.coloring().at_rest.bg_color);
    // t.render(&theme.coloring().at_rest, draw_text);
    macroquad::text::draw_text(
        test_text,
        0.0,
        theme.font_size() * 0.5,
        theme.font_size(),
        BLACK,
    );
    pop_camera_state();

    let rect = anchor.get_rect(t.rect().size());
    let pos = Coord::new_f(rect.x, z + 0.5).to_vec3(rect.bottom());
    mesh_vertical_texture(pos, 0.5, WHITE, Some(render_target.texture), false)
    // mesh_vertical_texture(pos, 0.5, WHITE, None)
}
*/
pub fn below_left(rect: Rect) -> Anchor {
    Anchor::below(rect, Horizontal::Left, 0.0)
}
pub fn draw_text(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    style: &StateStyle,
    font: Option<Font>,
) {
    let params = TextParams {
        font: font.unwrap_or(Font::default()),
        font_size: font_size as u16,
        color: style.text_color,
        font_scale: 1.0,
        ..TextParams::default()
    };
    macroquad::text::draw_text_ex(text, x, y, params)
}
