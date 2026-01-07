use crate::theme::Theme;
use juquad::widgets::text::draw_text;

pub fn render_text(text: &str, x: f32, y: f32, theme: &Theme) {
    draw_text(
        text,
        x,
        y + theme.font_size(),
        theme.font_size(),
        &theme.coloring().at_rest,
        Some(theme.font()),
    )
}
