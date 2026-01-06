use juquad::widgets::from_hexes;
use macroquad::color::Color;
use macroquad::color_u8;

pub struct Theme {
    pub palette: Palette,
}

pub struct Palette {
    pub white_cells: Color,
    pub black_cells: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            palette: Palette::default(),
        }
    }
}
impl Default for Palette {
    fn default() -> Self {
        Self {
            white_cells: from_hex(0xF7FFE5),
            black_cells: from_hex(0x181449),
        }
    }
}

pub const fn from_hex(hex: u32) -> Color {
    color_u8!(hex / 0x10000, hex / 0x100 % 0x100, hex % 0x100, 255)
}
