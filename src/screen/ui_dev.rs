use crate::core::array_union::ArrayUnionTrait;
use crate::core::clipboard::Clipboard;
use crate::core::time::Time;
use crate::screen::theme::{
    coloring_elem, named_coloring, named_state_style, new_coloring, set_theme_coloring,
    state_style_elem, CameraPos, Palette, Theme,
};
use crate::screen::ui;
use crate::screen::ui::{
    render_button_dev_mut, render_slider, render_text_dev, render_text_dev_mut, rightwards,
};
use crate::world::board::{Board, DEFAULT_PIECE_SIZE};
use crate::{AnyResult, INITIAL_DEV_UI};
use juquad::draw::draw_rect;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::Interaction;
use macroquad::color::Color;
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::info;
use std::io::Write;
use ui::below_left;

#[derive(Eq, PartialEq)]
pub enum DevUiMenu {
    Hidden,
    Main,
    Camera,
    Referee,
    PaletteWorld,
    PaletteUi,
    EditWorldColor(usize),
    EditUiColor(usize, usize),
}

pub struct DevUi {
    pub menu: DevUiMenu,
    pub copied_color_name: Option<(String, Color)>,
    pub clipboard: Clipboard,
}

impl DevUi {
    pub fn new() -> AnyResult<Self> {
        Ok(Self {
            menu: INITIAL_DEV_UI,
            copied_color_name: None,
            clipboard: Clipboard::new()?,
        })
    }
    pub fn toggle(&mut self) {
        self.menu = if self.show() {
            DevUiMenu::Hidden
        } else {
            DevUiMenu::Main
        };
    }
    pub fn show(&self) -> bool {
        self.menu != DevUiMenu::Hidden
    }
    pub fn draw(
        &mut self,
        time: &Time,
        theme: &mut Theme,
        board: &mut Board,
        camera: &mut CameraPos,
    ) -> AnyResult<()> {
        match self.menu {
            DevUiMenu::Hidden => {}
            DevUiMenu::Main => self.draw_main(theme),
            DevUiMenu::Camera => self.draw_camera(time, theme, board, camera),
            DevUiMenu::Referee => self.draw_referee(theme, board),
            DevUiMenu::PaletteWorld => self.draw_palette(theme)?,
            DevUiMenu::PaletteUi => self.draw_palette_ui(theme)?,
            DevUiMenu::EditWorldColor(index) => self.draw_edit_world_color(index, theme)?,
            DevUiMenu::EditUiColor(state_index, color_index) => {
                self.draw_edit_ui_color(state_index, color_index, theme)?
            }
        }
        Ok(())
    }
    fn dev_ui_title(theme: &mut Theme) -> Rect {
        render_text_dev(
            "DEV UI (toggle with '/')",
            theme,
            Anchor::top_left(0.0, 0.0),
        )
    }

    fn navigation(&mut self, theme: &Theme, text: &str, menu: DevUiMenu, rect: &mut Rect) -> Rect {
        self.navigation_anchor(theme, text, menu, below_left, rect);
        *rect
    }
    fn navigation_anchor(
        &mut self,
        theme: &Theme,
        text: &str,
        menu: DevUiMenu,
        anchor: fn(Rect) -> Anchor,
        rect: &mut Rect,
    ) {
        let back_clicked = render_button_dev_mut(text, theme, anchor, rect);
        if back_clicked.is_clicked() {
            self.menu = menu;
        }
    }

    fn draw_main(&mut self, theme: &mut Theme) {
        let rect = &mut Self::dev_ui_title(theme);
        self.navigation(theme, "Camera controls", DevUiMenu::Camera, rect);
        self.navigation(theme, "Inspect referee", DevUiMenu::Referee, rect);
        self.navigation(theme, "Edit world palette", DevUiMenu::PaletteWorld, rect);
        self.navigation(theme, "Edit ui palette", DevUiMenu::PaletteUi, rect);
        self.navigation(theme, "Hide Dev UI", DevUiMenu::Hidden, rect);
    }

    fn draw_camera(
        &mut self,
        time: &Time,
        theme: &mut Theme,
        board: &mut Board,
        camera: &mut CameraPos,
    ) {
        let rect = &mut Self::dev_ui_title(theme);
        // let text = "You can move the green cursor with your keyboard arrows";
        // let rect = render_text(text, below_left(rect), theme);
        // let rect = render_text("Toggle dev UI with '/'", below_left(rect), theme);
        let text = format!("FPS: {:.1}", time.fps());
        render_text_dev_mut(&text, theme, below_left, rect);
        // let text = format!("scale: {}", unsafe { SCALE });
        // let _rect = render_text(&text, below_left(_rect), theme);

        let _rect = render_slider(
            "Texture size X",
            theme,
            0.1,
            2.0,
            &mut board.piece_size.x,
            rect,
        );
        let _rect = render_slider(
            "Texture size Y",
            theme,
            0.1,
            2.0,
            &mut board.piece_size.y,
            rect,
        );
        let clicked = render_button_dev_mut("Reset texture size", theme, below_left, rect);
        if clicked.is_clicked() {
            board.piece_size = DEFAULT_PIECE_SIZE;
        }
        render_slider("Camera Y", theme, 0.0, 100.0, &mut camera.y, rect);
        render_slider("Camera Z", theme, 0.0, 100.0, &mut camera.z, rect);
        render_slider("Camera Width", theme, 43.5, 47.5, &mut camera.fovy, rect);
        render_slider(
            "Camera target Y",
            theme,
            -5.0,
            10.0,
            &mut camera.target_y,
            rect,
        );
        let clicked = render_button_dev_mut("Reset camera", theme, below_left, rect);
        if clicked.is_clicked() {
            *camera = CameraPos::default();
        }

        self.navigation(theme, "Back", DevUiMenu::Main, rect);
    }

    fn draw_referee(&mut self, theme: &mut Theme, board: &mut Board) {
        let rect = &mut Self::dev_ui_title(theme);
        let dir = board.referee.dir_c();
        render_text_dev_mut(
            &format!("Referee dir: {:0>5.2} {:0>5.2}", dir.column, dir.row),
            theme,
            below_left,
            rect,
        );
        render_text_dev_mut(
            &format!("trip time: {:0>5.2}", board.referee.trip_time),
            theme,
            below_left,
            rect,
        );
        self.navigation(theme, "Back", DevUiMenu::Main, rect);
    }
    fn draw_palette(&mut self, theme: &mut Theme) -> AnyResult<()> {
        self.clipboard.maybe_refresh()?;
        // let _rect = Self::dev_ui_title(theme);
        let title = self.palette_title()?;
        let mut rect = render_text_dev(&title, theme, Anchor::top_left(0.0, 0.0));
        for (index, (name, color)) in theme.palette.named_iter().enumerate() {
            if let Some(color) = self.copy_paste(name, color, theme, &mut rect)? {
                theme.palette.set(index, color);
            }
            draw_color_rect(color, &mut rect);
            let text = format!("{} - {}", as_hex(color), name);
            let menu = DevUiMenu::EditWorldColor(index);
            let mut rect_copy = rect;
            self.navigation_anchor(theme, &text, menu, rightwards, &mut rect_copy);
        }

        let text = "Export world palette source code (F12 in the browser to see)";
        if render_button_dev_mut(text, theme, below_left, &mut rect).is_clicked() {
            let code = palette_to_code(theme)?;
            info!("{}", code);
            self.clipboard.copy(code)?;
        }
        if render_button_dev_mut("Reset palette", theme, below_left, &mut rect).is_clicked() {
            theme.palette = Palette::default();
        }
        self.navigation(theme, "Back", DevUiMenu::Main, &mut rect);
        Ok(())
    }

    fn palette_title(&mut self) -> AnyResult<String> {
        let title = format!(
            "Colors (in RGBA){}",
            if let Some(copied) = self.clipboard.paste().and_then(parse_hex_color) {
                let color_string = as_hex(copied);
                if let Some((name, old_color)) = self.copied_color_name.as_ref() {
                    if *old_color == copied {
                        format!(" (copied {} - {})", color_string, name)
                    } else {
                        self.copied_color_name = None;
                        format!(" (copied {})", color_string)
                    }
                } else {
                    format!(" (copied {})", color_string)
                }
            } else {
                "".to_string()
            }
        );
        Ok(title)
    }

    fn draw_edit_world_color(&mut self, color_index: usize, theme: &mut Theme) -> AnyResult<()> {
        self.clipboard.maybe_refresh()?;
        let (name, mut color) = theme.palette.named_vec()[color_index];
        let rect = &mut Rect::default();
        if color_editor(theme, name, rect, &mut color).is_clicked() {
            (_, color) = Palette::default().named_vec()[color_index];
        }
        theme.palette.set(color_index, color);
        self.navigation(theme, "Back", DevUiMenu::PaletteWorld, rect);
        Ok(())
    }

    fn draw_palette_ui(&mut self, theme: &mut Theme) -> AnyResult<()> {
        self.clipboard.maybe_refresh()?;
        // let _rect = Self::dev_ui_title(theme);
        let title = self.palette_title()?;
        let mut rect = render_text_dev(&title, theme, Anchor::top_left(0.0, 0.0));
        for (state_i, (name, state_style)) in named_coloring(theme.coloring()).enumerate() {
            for (color_i, (color_name, color)) in named_state_style(state_style).enumerate() {
                let full_name = format!("{}/{}", name, color_name);
                if let Some(color) = self.copy_paste(&full_name, color, theme, &mut rect)? {
                    set_theme_coloring(color, state_i, color_i, theme);
                }
                draw_color_rect(color, &mut rect);
                let text = format!("{} - {}", as_hex(color), full_name);
                let menu = DevUiMenu::EditUiColor(state_i, color_i);
                let mut rect_copy = rect;
                self.navigation_anchor(theme, &text, menu, rightwards, &mut rect_copy);
            }
        }

        let text = "Export ui palette source code (F12 in the browser to see)";
        if render_button_dev_mut(text, theme, below_left, &mut rect).is_clicked() {
            let code = coloring_to_code(theme)?;
            info!("{}", code);
            self.clipboard.copy(code)?;
        };
        if render_button_dev_mut("Reset palette", theme, below_left, &mut rect).is_clicked() {
            theme.set_coloring(new_coloring());
        }
        self.navigation(theme, "Back", DevUiMenu::Main, &mut rect);
        Ok(())
    }
    fn draw_edit_ui_color(
        &mut self,
        state_style_index: usize,
        color_index: usize,
        theme: &mut Theme,
    ) -> AnyResult<()> {
        self.clipboard.maybe_refresh()?;
        let (name, state_style) = named_coloring(theme.coloring())
            .nth(state_style_index)
            .unwrap();
        let (color_name, mut color) = named_state_style(state_style).nth(color_index).unwrap();
        let rect = &mut Rect::default();
        let combined_name = format!("{}/{}", name, color_name);
        if color_editor(theme, &combined_name, rect, &mut color).is_clicked() {
            let style = coloring_elem(new_coloring(), state_style_index);
            color = state_style_elem(style, color_index);
        }
        set_theme_coloring(color, state_style_index, color_index, theme);
        self.navigation(theme, "Back", DevUiMenu::PaletteUi, rect);
        Ok(())
    }

    fn copy_paste(
        &mut self,
        name: &str,
        color: Color,
        theme: &Theme,
        rect: &mut Rect,
    ) -> AnyResult<Option<Color>> {
        if render_button_dev_mut("Copy", theme, below_left, rect).is_clicked() {
            let color_string = as_hex_no_space(color);
            self.copied_color_name =
                Some((name.to_string(), parse_hex_color(&color_string).unwrap()));
            self.clipboard.copy(color_string)?;
        }

        if let Some(copied) = self.clipboard.paste().and_then(parse_hex_color) {
            let mut copied_rect = rect.clone();
            let clicked = render_button_dev_mut("Paste", theme, rightwards, &mut copied_rect);
            *rect = rect.combine_with(copied_rect);
            if clicked.is_clicked() {
                return Ok(Some(copied));
            }
        } else {
            self.copied_color_name = None;
        }
        Ok(None)
    }
}

fn draw_color_rect(color: Color, previous_rect: &mut Rect) {
    let anchor = rightwards(*previous_rect);
    let size = Vec2::splat(previous_rect.h);
    let rect = anchor.get_rect(size);
    draw_rect(rect, color);
    *previous_rect = rect.combine_with(*previous_rect);
}

fn color_editor(theme: &mut Theme, name: &str, rect: &mut Rect, color: &mut Color) -> Interaction {
    let text = format!("Edit color '{}' {}", name, as_hex(*color));
    render_text_dev_mut(&text, theme, below_left, rect);
    draw_color_rect(*color, rect);
    // color = Color::new(color.r * 255.0, color.g * 255.0, color.b * 255.0, color.a * 255.0);
    render_slider("Red  ", theme, 0.0, 1.0, &mut color.r, rect);
    render_slider("Green", theme, 0.0, 1.0, &mut color.g, rect);
    render_slider("Blue ", theme, 0.0, 1.0, &mut color.b, rect);
    render_slider("Alpha", theme, 0.0, 1.0, &mut color.a, rect);

    // color = Color::new(color.r /255.0, color.g/255.0, color.b /255.0, color.a /255.0);
    render_button_dev_mut("Reset color", theme, below_left, rect)
}

pub fn as_hex(color: Color) -> String {
    let [r, g, b, a]: [u8; 4] = color.into();
    format!("0x {:0>2X} {:0>2X} {:0>2X} {:0>2X}", r, g, b, a)
}
pub fn as_hex_no_space(color: Color) -> String {
    let [r, g, b, a]: [u8; 4] = color.into();
    format!("0x{:0>2X}{:0>2X}{:0>2X}{:0>2X}", r, g, b, a)
}
pub fn parse_hex_color<S: AsRef<str>>(text: S) -> Option<Color> {
    let text = text.as_ref().to_lowercase();
    let view = if text.starts_with("0x") {
        &text[2..]
    } else if text.starts_with("#") {
        &text[1..]
    } else {
        &text
    };
    if view.len() == 8 {
        component_strings_to_color([&view[0..2], &view[2..4], &view[4..6], &view[6..8]])
    } else if view.len() == 6 {
        component_strings_to_color([&view[0..2], &view[2..4], &view[4..6], "ff"])
    } else if view.len() == 4 {
        let array = [d(view, 0)?, d(view, 1)?, d(view, 2)?, d(view, 3)?];
        Some(array.into())
    } else if view.len() == 3 {
        let array = [d(view, 0)?, d(view, 1)?, d(view, 2)?, 0xff];
        Some(array.into())
    } else {
        None
    }
}

fn d(view: &str, index: usize) -> Option<u8> {
    Some(duplicate_hex_digit(hex_value(view.as_bytes()[index])?))
}

pub fn component_strings_to_color(components_str: [&str; 4]) -> Option<Color> {
    let mut components = [0u8; 4];
    for (i, part) in components_str.iter().enumerate() {
        components[i] = parse_hex_u8(part)?
    }
    Some(components.into())
}
pub fn duplicate_hex_digit(value: u8) -> u8 {
    value * 16 + value
}
pub fn hex_value(byte: u8) -> Option<u8> {
    if byte >= b'0' && byte <= b'9' {
        Some(byte - b'0')
    } else if byte >= b'a' && byte <= b'f' {
        Some(byte - b'a' + 10)
    } else {
        None
    }
}
/// assumes the str is in lowercase
pub fn parse_hex_u8(text: &str) -> Option<u8> {
    if text.bytes().count() <= 2 {
        let mut accum: u8 = 0;
        for byte in text.bytes() {
            accum *= 16;
            accum += hex_value(byte)?;
        }
        Some(accum)
    } else {
        None
    }
}

pub fn palette_to_code(theme: &Theme) -> AnyResult<String> {
    let mut message: Vec<u8> = Vec::new();
    write!(
        message,
        "\nimpl Default for Palette {{
    fn default() -> Self {{
        Self {{
"
    )?;
    for (name, color) in theme.palette.named_iter() {
        write!(
            message,
            "            {}: from_hex_rgba({}),\n",
            name,
            as_hex_no_space(color)
        )?;
    }
    write!(
        message,
        "        }}
    }}
}}"
    )?;
    Ok(String::from_utf8(message)?)
}

pub fn coloring_to_code(theme: &Theme) -> AnyResult<String> {
    //     Ok("pub fn new_coloring() -> Coloring {
    //     Coloring {
    //         at_rest: StateStyle {
    //             bg_color: from_hex(0x190e34),
    //             text_color: from_hex(0xfafbf9),
    //             border_color: from_hex(0xfafbf9),
    //         },
    //         ..Default::default()
    //     }
    // }"
    //     .to_string());

    let mut message: Vec<u8> = Vec::new();
    writeln!(
        message,
        "\npub fn new_coloring() -> Coloring {{
    Coloring {{"
    )?;
    for (style_name, style) in named_coloring(theme.coloring()) {
        writeln!(message, "        {}: StateStyle {{", style_name)?;
        for (color_name, color) in named_state_style(style) {
            writeln!(
                message,
                "            {}: from_hex_rgba({}),",
                color_name,
                as_hex_no_space(color)
            )?;
        }

        writeln!(message, "        }},")?;
    }
    // for (name, color) in theme.palette.named_iter() {
    // }
    write!(
        message,
        "    }}
}}"
    )?;
    Ok(String::from_utf8(message)?)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        fn reformat(s: &str) -> Option<String> {
            if let Some(parsed) = parse_hex_color(s) {
                Some(as_hex_no_space(parsed))
            } else {
                None
            }
        }
        assert_eq!(reformat(""), None);
        assert_eq!(reformat("nocolor"), None);
        assert_eq!(reformat("0x112233FF"), Some("0x112233FF".to_string()));
        assert_eq!(reformat("0x112233"), Some("0x112233FF".to_string()));
        assert_eq!(reformat("112233"), Some("0x112233FF".to_string()));
        assert_eq!(reformat("112233FF"), Some("0x112233FF".to_string()));
        assert_eq!(reformat("abc"), Some("0xAABBCCFF".to_string()));
        assert_eq!(reformat("abcd"), Some("0xAABBCCDD".to_string()));
        assert_eq!(reformat("#D9D7E8"), Some("0xD9D7E8FF".to_string()));
    }
}
