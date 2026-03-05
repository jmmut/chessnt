use crate::screen::theme::{margin, Theme};
use juquad::widgets::anchor::{Anchor, Layout};
use macroquad::math::{vec2, Rect};

pub fn inside_initial(theme: &Theme, screen: Rect, layout: Layout) -> Rect {
    Anchor::inside(screen, layout, margin(theme)).get_rect(vec2(0.0, 0.0))
}
