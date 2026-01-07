use crate::theme::Theme;
use juquad::draw::draw_rect;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::text::TextRect;
use juquad::widgets::Widget;
use macroquad::math::Rect;

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

pub fn below_left(rect: Rect) -> Anchor {
    Anchor::below(rect, Horizontal::Left, 0.0)
}
