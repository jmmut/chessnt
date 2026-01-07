use crate::{
    height_to_width, DEFAULT_ASPECT_RATIO, DEFAULT_FONT_SIZE, DEFAULT_WINDOW_HEIGHT,
    DEFAULT_WINDOW_WIDTH,
};
use macroquad::color::Color;
use macroquad::color_u8;
use macroquad::prelude::{Font, Texture2D, Vec2};

use juquad::widgets::Style as Coloring;

pub struct Theme {
    pub palette: Palette,
    base_font_size: f32,
    font_size: f32,
    font: Font,
    coloring: Coloring,
    pub textures: Textures,
}

impl Theme {
    pub fn new(textures: Textures) -> Self {
        Self {
            palette: Palette::default(),
            base_font_size: DEFAULT_FONT_SIZE,
            font_size: choose_scale(
                DEFAULT_WINDOW_WIDTH as f32,
                DEFAULT_WINDOW_HEIGHT as f32,
                DEFAULT_FONT_SIZE,
            ),
            font: Font::default(),
            coloring: Coloring::default(),
            textures,
        }
    }
    pub fn update_screen_size(&mut self, screen: Vec2) {
        self.font_size = choose_scale(screen.x, screen.y, self.base_font_size);
    }
    pub fn set_font(&mut self, font: Font) {
        self.font = font;
    }
    pub fn font(&self) -> Font {
        self.font
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn coloring(&self) -> Coloring {
        self.coloring
    }
}

pub struct Textures {
    pub placeholder: Texture2D,
}

pub struct Palette {
    pub white_cells: Color,
    pub black_cells: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            white_cells: from_hex(0xF7FFE5),
            black_cells: from_hex(0x181449),
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
