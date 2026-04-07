use crate::core::array_union::{ArrayUnion, ArrayUnionTrait, ExternalArrayUnion};
use crate::screen::shader::{Materials, character_shader};
use crate::world::moves::Move;
use crate::world::team::Team;
use crate::{
    AnyResult, DEFAULT_ASPECT_RATIO, DEFAULT_FONT_SIZE, DEFAULT_WINDOW_HEIGHT,
    DEFAULT_WINDOW_WIDTH, TRANSPARENT, height_to_width,
};
use juquad::draw::to_rect;
use juquad::elm::style::Style;
use juquad::lazy::{Margin, Pad, Size};
use juquad::widgets::anchor::{Horizontal, Layout, Vertical};
use juquad::widgets::{StateColor, StateStyle, Style as Coloring};
use macroquad::color::{BLUE, Color, DARKBLUE, GRAY, GREEN, LIGHTGRAY, PURPLE, RED, YELLOW};
use macroquad::color_u8;
use macroquad::math::{Rect, vec2};
use macroquad::prelude::{Font, Texture2D, Vec2};
use std::collections::HashMap;
use std::fs::read_to_string;

pub struct Theme {
    pub screen: Vec2,
    pub palette: Palette,
    base_font_size: f32,
    font_size: f32,
    fonts: Fonts,
    coloring: AllColoring,
    pub textures: Textures,
    pub materials: Materials,
}
pub struct RefreshShaders {
    pub character: bool,
}

impl Theme {
    pub fn new(textures: Textures, fonts: Fonts, materials: Materials) -> Self {
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
            coloring: new_coloring(),
            textures,
            materials,
        }
    }
    pub fn update_screen_size(&mut self, screen: Vec2) {
        self.screen = screen;
        self.font_size = choose_scale(screen.x, screen.y, self.base_font_size);
    }
    pub fn tick(&mut self, _delta_s: f64, screen: Vec2) -> AnyResult<()> {
        self.update_screen_size(screen);
        if self.materials.refresh_shaders.character {
            let reloaded = character_shader(
                &read_to_string("src/shaders/character_vertex.glsl")?,
                &read_to_string("src/shaders/character_fragment.glsl")?,
            );
            match reloaded {
                Ok(ok) => {
                    self.materials.character = ok;
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        Ok(())
    }
    pub fn screen_rect(&self) -> Rect {
        to_rect(vec2(0.0, 0.0), self.screen)
    }
    pub fn font(&self) -> &Font {
        &self.fonts.text
    }
    pub fn font_title(&self) -> &Font {
        &self.fonts.titles
    }
    pub fn font_dev(&self) -> &Font {
        &self.fonts.dev
    }
    pub fn font_size(&self) -> f32 {
        self.font_size
    }
    pub fn font_size_title(&self) -> f32 {
        self.font_size * 1.5
    }
    pub fn font_size_dev(&self) -> f32 {
        self.font_size * 0.92
    }
    pub fn coloring(&self) -> AllColoring {
        self.coloring
    }
    pub fn button_coloring(&self) -> Coloring {
        self.coloring.button_coloring
    }
    fn text_all_coloring(&self) -> Coloring {
        let coloring = self.coloring();
        Coloring {
            at_rest: coloring.text_coloring,
            ..coloring.button_coloring
        }
    }
    pub fn set_coloring(&mut self, coloring: AllColoring) {
        self.coloring = coloring;
    }
    pub fn set_button_coloring(&mut self, coloring: Coloring) {
        self.coloring.button_coloring = coloring;
    }
    pub fn title_style(&self) -> Style {
        Style {
            font_size: self.font_size_title(),
            font: Some(self.font_title().clone()),
            coloring: self.text_all_coloring(),
            ..self.button_style()
        }
    }

    pub fn container_style(&self) -> Style {
        let coloring = self.coloring();
        Style {
            pad: Pad::new(self.font_size() * 1.0, self.font_size() * 1.0),
            margin: Margin::new(self.font_size() * 1.0, self.font_size() * 1.0),
            border: 4.0,
            coloring: Coloring {
                at_rest: StateColor {
                    border_color: coloring.button_coloring.at_rest.bg_color,
                    ..coloring.text_coloring
                },
                ..coloring.button_coloring
            },
            layout: Layout::vertical(Vertical::Bottom, Horizontal::Center),
            ..Style::default()
        }
    }
    pub fn button_style(&self) -> Style {
        Style {
            pad: Pad::new(self.font_size() * 1.5, self.font_size() * 0.7),
            margin: Margin::new(self.font_size() * 1.0, self.font_size() * 1.0),
            border: 1.0,
            layout: Layout::vertical(Vertical::Bottom, Horizontal::Center),
            font_size: self.font_size(),
            font: Some(self.font().clone()),
            size: Size::Grow,
            coloring: self.coloring().button_coloring,
        }
    }
}
pub struct Textures {
    pub placeholder: Texture2D,
    pub referee: Texture2D,
    pub characters: HashMap<Move, Texture2D>,
    pub pieces: HashMap<(Team, Move), Texture2D>,
}

pub fn new_text_coloring() -> StateStyle {
    StateStyle {
        bg_color: from_hex_rgba(0x181449FF),
        text_color: from_hex_rgba(0xFAFBF9FF),
        border_color: from_hex_rgba(0x090E4BFF),
    }
}
pub fn new_button_coloring() -> Coloring {
    Coloring {
        at_rest: StateStyle {
            bg_color: from_hex_rgba(0x434367FF),
            text_color: from_hex_rgba(0xFAFBF9FF),
            border_color: from_hex_rgba(0x090E4BFF),
        },
        hovered: StateStyle {
            bg_color: from_hex_rgba(0xE0DCD3FF),
            text_color: from_hex_rgba(0x190E34FF),
            border_color: from_hex_rgba(0xA8AAA1FF),
        },
        pressed: StateStyle {
            bg_color: from_hex_rgba(0x181449FF),
            text_color: from_hex_rgba(0xFAFBF9FF),
            border_color: from_hex_rgba(0x090E4BFF),
        },
    }
}
pub fn new_coloring() -> AllColoring {
    AllColoring {
        text_coloring: new_text_coloring(),
        button_coloring: new_button_coloring(),
    }
}

#[derive(Copy, Clone)]
pub struct AllColoring {
    pub text_coloring: StateStyle,
    pub button_coloring: Coloring,
}

const COLORING_COUNT: usize = size_of::<AllColoring>() / size_of::<StateStyle>();
const COLORING_NAMES: [&str; COLORING_COUNT] = ["text", "at_rest", "hovered", "pressed"];

pub type ColoringUnion = ArrayUnion<AllColoring, StateStyle, COLORING_COUNT>;

pub fn named_coloring(coloring: AllColoring) -> impl Iterator<Item = (&'static str, StateStyle)> {
    ColoringUnion::named_iter(coloring, COLORING_NAMES)
}
pub fn coloring_elem(coloring: AllColoring, index: usize) -> StateStyle {
    ColoringUnion::array(coloring)[index]
}
pub fn set_coloring(coloring: &mut AllColoring, index: usize, color: StateStyle) {
    ColoringUnion::set(coloring, index, color)
}

const STATE_STYLE_COUNT: usize = size_of::<StateStyle>() / size_of::<Color>();
const STATE_STYLE_NAMES: [&str; STATE_STYLE_COUNT] = ["bg_color", "text_color", "border_color"];
pub type StateStyleUnion = ArrayUnion<StateStyle, Color, STATE_STYLE_COUNT>;

pub fn named_state_style(state_style: StateStyle) -> impl Iterator<Item = (&'static str, Color)> {
    StateStyleUnion::named_iter(state_style, STATE_STYLE_NAMES)
}
pub fn state_style_elem(state_style: StateStyle, index: usize) -> Color {
    StateStyleUnion::array(state_style)[index]
}
pub fn set_state_style(state_style: &mut StateStyle, index: usize, color: Color) {
    StateStyleUnion::set(state_style, index, color)
}
pub fn set_theme_coloring(
    color: Color,
    state_style_index: usize,
    color_index: usize,
    theme: &mut Theme,
) {
    let mut coloring = theme.coloring();
    let mut style = coloring_elem(coloring, state_style_index);
    set_state_style(&mut style, color_index, color);
    set_coloring(&mut coloring, state_style_index, style);
    theme.set_coloring(coloring);
}

#[derive(Copy, Clone)]
pub struct Palette {
    pub tiles_white: Color,
    pub tiles_black: Color,
    pub cursor_white: Color,
    pub cursor_black: Color,
    pub mask_white: Color,
    pub mask_black: Color,
    pub background: Color,
    pub spotlight: Color,
    pub radar: Color,
    pub ghost: Color,
    pub selection: Color,
    pub check: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            tiles_white: from_hex_rgba(0xBBADC4FF),
            tiles_black: from_hex_rgba(0x312C79FF),
            cursor_white: color_average_weight(color_average(GREEN, GRAY), YELLOW, 0.3),
            cursor_black: color_average_weight(color_average(GREEN, GRAY), DARKBLUE, 0.3),
            mask_white: from_hex_rgba(0x75B2F4FF),
            mask_black: from_hex_rgba(0x267013FF),
            background: from_hex_rgba(0x381C73FF),
            spotlight: color_average(GREEN, LIGHTGRAY),
            radar: color_average(RED, TRANSPARENT),
            ghost: color_average(PURPLE, GRAY),
            selection: color_average(BLUE, GRAY),
            check: color_average(RED, GRAY),
        }
    }
}
impl Palette {
    const COUNT: usize = size_of::<Palette>() / size_of::<Color>();
}
impl ArrayUnionTrait<Palette, Color, { Palette::COUNT }> for Palette {
    type Delegate = ArrayUnion<Palette, Color, { Palette::COUNT }>;
    const NAMES: [&'static str; Self::COUNT] = [
        "tiles_white",
        "tiles_black",
        "cursor_white",
        "cursor_black",
        "mask_white",
        "mask_black",
        "background",
        "spotlight",
        "radar",
        "ghost",
        "selection",
        "check",
    ];

    fn myself(self) -> Palette {
        self
    }
    fn myself_mut(&mut self) -> &mut Palette {
        self
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
pub fn pad(theme: &Theme) -> Vec2 {
    Vec2::splat(theme.font_size() * 2.0)
}
pub struct Fonts {
    pub titles: Font,
    pub text: Font,
    pub dev: Font,
}
impl Fonts {
    pub fn new(titles: Font, text: Font, dev: Font) -> Self {
        Self { titles, text, dev }
    }
}
pub const fn from_hex(hex: u32) -> Color {
    color_u8!(hex / 0x10000, hex / 0x100 % 0x100, hex % 0x100, 255)
}
pub const fn from_hex_rgba(hex: u32) -> Color {
    color_u8!(
        hex / 0x1000000,
        hex / 0x10000 % 0x100,
        hex / 0x100 % 0x100,
        hex % 0x100
    )
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
