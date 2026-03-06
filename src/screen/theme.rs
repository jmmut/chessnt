use crate::world::moves::Move;
use crate::world::team::Team;
use crate::{
    height_to_width, DEFAULT_ASPECT_RATIO, DEFAULT_FONT_SIZE, DEFAULT_WINDOW_HEIGHT,
    DEFAULT_WINDOW_WIDTH, TRANSPARENT,
};
use juquad::draw::to_rect;
use juquad::widgets::{StateStyle, Style as Coloring};
use macroquad::color::{Color, BLUE, DARKBLUE, GRAY, GREEN, PURPLE, RED, YELLOW};
use macroquad::color_u8;
use macroquad::math::{vec2, Rect};
use macroquad::prelude::{Font, Texture2D, Vec2};
use std::collections::HashMap;

pub struct Theme {
    pub screen: Vec2,
    pub palette: Palette,
    base_font_size: f32,
    font_size: f32,
    fonts: Fonts,
    coloring: Coloring,
    pub textures: Textures,
}

impl Theme {
    pub fn new(textures: Textures, fonts: Fonts) -> Self {
        Self {
            screen: vec2(1.0, 1.0),
            palette: Palette::default(),
            base_font_size: DEFAULT_FONT_SIZE,
            font_size: choose_scale(
                DEFAULT_WINDOW_WIDTH as f32,
                DEFAULT_WINDOW_HEIGHT as f32,
                DEFAULT_FONT_SIZE,
            ),
            fonts,
            coloring: Coloring {
                at_rest: StateStyle {
                    bg_color: from_hex(0x190e34),
                    text_color: from_hex(0xfafbf9),
                    border_color: from_hex(0xfafbf9),
                },
                ..Default::default()
            },
            textures,
        }
    }
    pub fn update_screen_size(&mut self, screen: Vec2) {
        self.screen = screen;
        self.font_size = choose_scale(screen.x, screen.y, self.base_font_size);
    }
    pub fn screen_rect(&self) -> Rect {
        to_rect(vec2(0.0, 0.0), self.screen)
    }
    pub fn font(&self) -> Font {
        self.fonts.text
    }
    pub fn font_title(&self) -> Font {
        self.fonts.titles
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn font_size_title(&self) -> f32 {
        self.font_size * 1.5
    }
    pub fn coloring(&self) -> Coloring {
        self.coloring
    }
}
pub struct CameraPos {
    pub y: f32,
    pub z: f32,
    pub fovy: f32,
    pub target_y: f32,
}
pub struct Textures {
    pub placeholder: Texture2D,
    pub pieces: HashMap<(Team, Move), Texture2D>,
}

pub struct Palette {
    pub white_tiles: Color,
    pub black_tiles: Color,
    pub radar: Color,
    pub ghost: Color,
    pub selection: Color,
    pub check: Color,
    pub cursor_white: Color,
    pub cursor_black: Color,
}

// // const SELECTION: Color = color_average(DARKBLUE, TRANSPARENT);
// const SELECTION: Color = color_average(BLUE, GRAY);
// // const RADAR: Color = color_average(color_average(RED, LIGHTGRAY), TRANSPARENT);
// pub const RADAR: Color = color_average(RED, TRANSPARENT);
// // const GHOST: Color = color_average(DARKPURPLE, TRANSPARENT);
// const GHOST: Color = color_average(PURPLE, GRAY);
// const CHECK: Color = color_average(RED, GRAY);
// // const CURSOR: Color = color_average(DARKGREEN, TRANSPARENT);
// const CURSOR_WHITE: Color = color_average_weight(color_average(GREEN, GRAY), YELLOW, 0.3);
// const CURSOR_BLACK: Color = color_average_weight(color_average(GREEN, GRAY), DARKBLUE, 0.3);
// // const FIGURE: Color = color_average(PINK, TRANSPARENT);
impl Default for Palette {
    fn default() -> Self {
        Self {
            white_tiles: from_hex(0xF7FFE5),
            black_tiles: from_hex(0x181449),
            radar: color_average(RED, TRANSPARENT),
            ghost: color_average(PURPLE, GRAY),
            selection: color_average(BLUE, GRAY),
            check: color_average(RED, GRAY),
            cursor_white: color_average_weight(color_average(GREEN, GRAY), YELLOW, 0.3),
            cursor_black: color_average_weight(color_average(GREEN, GRAY), DARKBLUE, 0.3),
        }
    }
}
const fn choose_scale(width: f32, height: f32, font_size: f32) -> f32 {
    let min_side = width.min(height_to_width(height, DEFAULT_ASPECT_RATIO));
    font_size
        * if min_side < 1600.0 {
            1.0
        } else if min_side < 2500.0 {
            1.5
        } else {
            2.0
        }
}

pub fn margin(theme: &Theme) -> Vec2 {
    Vec2::splat(theme.font_size() * 2.0)
}
pub struct Fonts {
    pub titles: Font,
    pub text: Font,
}
impl Fonts {
    pub fn new(titles: Font, text: Font) -> Self {
        Self { titles, text }
    }
}
pub const fn from_hex(hex: u32) -> Color {
    color_u8!(hex / 0x10000, hex / 0x100 % 0x100, hex % 0x100, 255)
}
pub const fn color_average(color_1: Color, color_2: Color) -> Color {
    color_average_weight(color_1, color_2, 0.5)
}
pub const fn color_average_weight(color_1: Color, color_2: Color, weight: f32) -> Color {
    Color::new(
        color_1.r * (1.0 - weight) + color_2.r * weight,
        color_1.g * (1.0 - weight) + color_2.g * weight,
        color_1.b * (1.0 - weight) + color_2.b * weight,
        color_1.a * (1.0 - weight) + color_2.a * weight,
    )
}
