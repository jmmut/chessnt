use crate::screen::theme::{margin, Theme};
use juquad::widgets::anchor::{Anchor, Layout};
use macroquad::math::{vec2, Rect, Vec2};

pub fn inside_initial(theme: &Theme, screen: Rect, layout: Layout) -> Rect {
    inside_initial_pad(screen, layout, margin(theme))
}
pub fn inside_initial_pad(screen: Rect, layout: Layout, pad: Vec2) -> Rect {
    Anchor::inside(screen, layout, pad).get_rect(vec2(0.0, 0.0))
}
