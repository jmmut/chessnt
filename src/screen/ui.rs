use crate::screen::theme::Theme;
use juquad::draw::draw_rect;
use juquad::input::input_macroquad::InputMacroquad;
use juquad::lazy::{Interactable, Renderable, Style, WidgetTrait};
use juquad::widgets::anchor::{Anchor, Horizontal, Vertical};
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use juquad::widgets::{Interaction, StateStyle, Widget};
use macroquad::math::Rect;
use macroquad::prelude::{Font, TextParams};

pub fn render_text(text: &str, theme: &Theme, anchor: Anchor) -> Rect {
    render_text_font(text, theme, theme.font(), anchor)
}
pub fn render_title(text: &str, theme: &Theme, anchor: Anchor) -> Rect {
    render_text_font_size(
        text,
        theme,
        theme.font_title(),
        theme.font_size_title(),
        anchor,
    )
}
pub fn render_text_dev(text: &str, theme: &Theme, anchor: Anchor) -> Rect {
    render_text_font_size(text, theme, theme.font_dev(), theme.font_size_dev(), anchor)
}
pub fn render_text_dev_mut(text: &str, theme: &Theme, anchor: fn(Rect) -> Anchor, rect: &mut Rect) {
    *rect = render_text_font_size(
        text,
        theme,
        theme.font_dev(),
        theme.font_size_dev(),
        anchor(*rect),
    );
}
pub fn render_text_font(text: &str, theme: &Theme, font: &Font, anchor: Anchor) -> Rect {
    render_text_font_size(text, theme, font, theme.font_size(), anchor)
}
pub fn render_text_no_font(
    text: &str,
    font_size: f32,
    coloring: StateStyle,
    anchor: Anchor,
) -> Rect {
    let t = TextRect::new_generic(
        text,
        anchor,
        font_size,
        None,
        macroquad::prelude::measure_text,
    );
    draw_rect(t.rect(), coloring.bg_color);
    t.render_default(&coloring);
    t.rect()
}
pub fn render_text_font_size(
    text: &str,
    theme: &Theme,
    font: &Font,
    font_size: f32,
    anchor: Anchor,
) -> Rect {
    let t = TextRect::new_generic(
        text,
        anchor,
        font_size,
        Some(font),
        macroquad::prelude::measure_text,
    );
    draw_rect(t.rect(), theme.coloring().text_coloring.bg_color);
    t.render_default(&theme.coloring().text_coloring);
    t.rect()
}

pub fn measure_title(text: &str, theme: &Theme, anchor: Anchor) -> TextRect {
    TextRect::new_generic(
        text,
        anchor,
        theme.font_size_title(),
        Some(theme.font_title()),
        macroquad::prelude::measure_text,
    )
}

pub fn render_button(text: &str, theme: &Theme, anchor: Anchor) -> (Rect, Interaction) {
    render_button_font(text, theme, theme.font(), theme.font_size(), anchor)
}
pub fn render_button_dev(text: &str, theme: &Theme, anchor: Anchor) -> (Rect, Interaction) {
    render_button_font(text, theme, theme.font_dev(), theme.font_size_dev(), anchor)
}
pub fn render_button_dev_mut(
    text: &str,
    theme: &Theme,
    anchor: fn(Rect) -> Anchor,
    rect: &mut Rect,
) -> Interaction {
    render_button_font_mut(
        text,
        theme,
        theme.font_dev(),
        theme.font_size_dev(),
        anchor,
        rect,
    )
}
pub fn render_button_font(
    text: &str,
    theme: &Theme,
    font: &Font,
    font_size: f32,
    anchor: Anchor,
) -> (Rect, Interaction) {
    let mut t = Button::new_generic(
        text,
        anchor,
        font_size,
        Some(font),
        macroquad::prelude::measure_text,
        Box::new(InputMacroquad),
    );
    let interaction = t.interact();
    t.render_default(&theme.button_coloring());
    (t.rect(), interaction)
}
pub fn render_button_font_mut(
    text: &str,
    theme: &Theme,
    font: &Font,
    font_size: f32,
    anchor: fn(Rect) -> Anchor,
    rect: &mut Rect,
) -> Interaction {
    let mut t = Button::new_generic(
        text,
        anchor(*rect),
        font_size,
        Some(font),
        macroquad::prelude::measure_text,
        Box::new(InputMacroquad),
    );
    let interaction = t.interact();
    t.render_default(&theme.button_coloring());
    *rect = t.rect();
    interaction
}

pub fn measure_button(text: &str, theme: &Theme, anchor: Anchor) -> Button {
    Button::new_generic(
        text,
        anchor,
        theme.font_size(),
        Some(theme.font()),
        macroquad::prelude::measure_text,
        Box::new(InputMacroquad),
    )
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
pub fn below_center(rect: Rect) -> Anchor {
    Anchor::below(rect, Horizontal::Center, 0.0)
}
pub fn rightwards(rect: Rect) -> Anchor {
    Anchor::rightwards(rect, Vertical::Center, 0.0)
}
pub fn draw_text(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    style: &StateStyle,
    font: Option<&Font>,
) {
    let params = TextParams {
        font,
        font_size: font_size as u16,
        color: style.text_color,
        font_scale: 1.0,
        ..TextParams::default()
    };
    macroquad::text::draw_text_ex(text, x, y, params);
}

pub fn render_slider(
    text: &str,
    theme: &Theme,
    min: f32,
    max: f32,
    value: &mut f32,
    rect: &mut Rect,
) {
    render_slider_mut(text, theme, min, max, value, below_left, rect)
}

pub fn render_slider_mut(
    text: &str,
    theme: &Theme,
    min: f32,
    max: f32,
    value: &mut f32,
    anchor: fn(Rect) -> Anchor,
    rect: &mut Rect,
) {
    let text_rect = render_text_dev(&format!("{}: {:0>5.2}", text, value), theme, anchor(*rect));
    let mut style = Style::default();
    style.coloring = theme.button_coloring();
    let mut slider = juquad::lazy::slider::Slider::new(style, min, max, *value);
    slider.set_pos(rightwards(text_rect).get_top_left_pixel(slider.size()));
    *value = *(slider
        .interact()
        .into_iter()
        .next()
        .unwrap()
        .downcast::<f32>()
        .unwrap());
    slider.render_interactive(Interaction::None);
    *rect = slider.rect().combine_with(text_rect);
}
