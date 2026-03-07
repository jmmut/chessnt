use crate::core::array_union::ArrayUnionTrait;
use crate::core::time::Time;
use crate::screen::theme::{
    coloring_elem, named_coloring, named_state_style, new_coloring, set_coloring, set_state_style,
    state_style_elem, CameraPos, Palette, Theme,
};
use crate::screen::ui;
use crate::screen::ui::{render_button_dev, render_slider, render_text_dev};
use crate::world::board::{Board, DEFAULT_PIECE_SIZE};
use crate::{AnyResult, INITIAL_DEV_UI};
use juquad::widgets::anchor::Anchor;
use juquad::widgets::Interaction;
use macroquad::color::Color;
use macroquad::math::Rect;
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
}

impl DevUi {
    pub fn new() -> Self {
        Self {
            menu: INITIAL_DEV_UI,
        }
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
            DevUiMenu::EditWorldColor(index) => self.draw_edit_world_color(index, theme),
            DevUiMenu::EditUiColor(state_index, color_index) => {
                self.draw_edit_ui_color(state_index, color_index, theme)
            }
        }
        Ok(())
    }
    fn dev_ui_title(theme: &mut Theme) -> Rect {
        let _rect = render_text_dev(
            "DEV UI (toggle with '/')",
            Anchor::top_left(0.0, 0.0),
            theme,
        );
        _rect
    }

    fn navigation(&mut self, theme: &mut Theme, _rect: Rect, text: &str, menu: DevUiMenu) -> Rect {
        let (rect, back_clicked) = render_button_dev(text, below_left(_rect), theme);
        if back_clicked.is_clicked() {
            self.menu = menu;
        }
        rect
    }

    fn draw_main(&mut self, theme: &mut Theme) {
        let _rect = Self::dev_ui_title(theme);
        let _rect = self.navigation(theme, _rect, "Camera controls", DevUiMenu::Camera);
        let _rect = self.navigation(theme, _rect, "Inspect referee", DevUiMenu::Referee);
        let _rect = self.navigation(theme, _rect, "Edit world palette", DevUiMenu::PaletteWorld);
        let _rect = self.navigation(theme, _rect, "Edit ui palette", DevUiMenu::PaletteUi);
        let _rect = self.navigation(theme, _rect, "Hide Dev UI", DevUiMenu::Hidden);
    }

    fn draw_camera(
        &mut self,
        time: &Time,
        theme: &mut Theme,
        board: &mut Board,
        camera: &mut CameraPos,
    ) {
        let _rect = Self::dev_ui_title(theme);
        // let text = "You can move the green cursor with your keyboard arrows";
        // let rect = render_text(text, below_left(rect), theme);
        // let rect = render_text("Toggle dev UI with '/'", below_left(rect), theme);
        let text = format!("FPS: {:.1}", time.fps());
        let _rect = render_text_dev(&text, below_left(_rect), theme);
        // let text = format!("scale: {}", unsafe { SCALE });
        // let _rect = render_text(&text, below_left(_rect), theme);

        let _rect = render_slider(
            "Texture size X",
            _rect,
            theme,
            &mut board.piece_size.x,
            0.1,
            2.0,
        );
        let _rect = render_slider(
            "Texture size Y",
            _rect,
            theme,
            &mut board.piece_size.y,
            0.1,
            2.0,
        );
        let (_rect, clicked) = render_button_dev("Reset texture size", below_left(_rect), theme);
        if clicked.is_clicked() {
            board.piece_size = DEFAULT_PIECE_SIZE;
        }
        let _rect = render_slider("Camera Y", _rect, theme, &mut camera.y, 0.0, 100.0);
        let _rect = render_slider("Camera Z", _rect, theme, &mut camera.z, 0.0, 100.0);
        let _rect = render_slider("Camera Width", _rect, theme, &mut camera.fovy, 43.5, 47.5);
        let _rect = render_slider(
            "Camera target Y",
            _rect,
            theme,
            &mut camera.target_y,
            -5.0,
            10.0,
        );
        let (_rect, clicked) = render_button_dev("Reset camera", below_left(_rect), theme);
        if clicked.is_clicked() {
            *camera = CameraPos::default();
        }

        self.navigation(theme, _rect, "Back", DevUiMenu::Main);
    }

    fn draw_referee(&mut self, theme: &mut Theme, board: &mut Board) {
        let _rect = Self::dev_ui_title(theme);
        let dir = board.referee.dir_c();
        let _rect = render_text_dev(
            &format!("Referee dir: {:0>5.2} {:0>5.2}", dir.column, dir.row),
            below_left(_rect),
            theme,
        );
        let _rect = render_text_dev(
            &format!("trip time: {:0>5.2}", board.referee.trip_time),
            below_left(_rect),
            theme,
        );
        self.navigation(theme, _rect, "Back", DevUiMenu::Main);
    }
    fn draw_palette(&mut self, theme: &mut Theme) -> AnyResult<()> {
        // let _rect = Self::dev_ui_title(theme);
        let mut _rect = render_text_dev("Colors (in RGBA)", Anchor::top_left(0.0, 0.0), theme);
        for (index, (name, color)) in theme.palette.named_iter().enumerate() {
            let menu = DevUiMenu::EditWorldColor(index);
            let text = format!("{} - {}", as_hex(color), name);
            _rect = self.navigation(theme, _rect, &text, menu);
        }

        let (_rect, clicked) = render_button_dev(
            "Export palette (see in browser with F12)",
            below_left(_rect),
            theme,
        );
        if clicked.is_clicked() {
            info!("{}", palette_to_code(theme)?);
        }
        let (_rect, clicked) = render_button_dev("Reset palette", below_left(_rect), theme);
        if clicked.is_clicked() {
            theme.palette = Palette::default();
        }
        self.navigation(theme, _rect, "Back", DevUiMenu::Main);
        Ok(())
    }
    fn draw_edit_world_color(&mut self, color_index: usize, theme: &mut Theme) {
        let (name, mut color) = theme.palette.named_vec()[color_index];
        let (_rect, clicked) = color_editor(theme, name, &mut color);
        if clicked.is_clicked() {
            (_, color) = Palette::default().named_vec()[color_index];
        }
        theme.palette.set(color_index, color);
        self.navigation(theme, _rect, "Back", DevUiMenu::PaletteWorld);
    }

    fn draw_palette_ui(&mut self, theme: &mut Theme) -> AnyResult<()> {
        // let _rect = Self::dev_ui_title(theme);
        let mut _rect = render_text_dev("Colors (in RGBA)", Anchor::top_left(0.0, 0.0), theme);
        for (state_i, (name, state_style)) in named_coloring(theme.coloring()).enumerate() {
            for (color_i, (color_name, color)) in named_state_style(state_style).enumerate() {
                let menu = DevUiMenu::EditUiColor(state_i, color_i);
                let text = format!("{} - {}/{}", as_hex_no_space(color), name, color_name);
                _rect = self.navigation(theme, _rect, &text, menu);
            }
        }

        let (_rect, clicked) = render_button_dev(
            "Export palette (see in browser with F12)",
            below_left(_rect),
            theme,
        );
        if clicked.is_clicked() {
            info!("{}", coloring_to_code(theme)?);
        }
        let (_rect, clicked) = render_button_dev("Reset palette", below_left(_rect), theme);
        if clicked.is_clicked() {
            theme.set_coloring(new_coloring());
        }
        self.navigation(theme, _rect, "Back", DevUiMenu::Main);
        Ok(())
    }
    fn draw_edit_ui_color(
        &mut self,
        state_style_index: usize,
        color_index: usize,
        theme: &mut Theme,
    ) {
        let (name, state_style) = named_coloring(theme.coloring())
            .nth(state_style_index)
            .unwrap();
        let (color_name, mut color) = named_state_style(state_style).nth(color_index).unwrap();
        let (_rect, clicked) = color_editor(theme, &format!("{}/{}", name, color_name), &mut color);
        if clicked.is_clicked() {
            let style = coloring_elem(new_coloring(), state_style_index);
            color = state_style_elem(style, color_index);
        }
        let mut coloring = theme.coloring();
        let mut style = coloring_elem(coloring, state_style_index);
        set_state_style(&mut style, color_index, color);
        set_coloring(&mut coloring, state_style_index, style);
        theme.set_coloring(coloring);
        self.navigation(theme, _rect, "Back", DevUiMenu::PaletteUi);
    }
}

fn color_editor(theme: &mut Theme, name: &str, color: &mut Color) -> (Rect, Interaction) {
    let text = format!("Edit color '{}' {}", name, as_hex(*color));
    let mut _rect = render_text_dev(&text, Anchor::top_left(0.0, 0.0), theme);
    // color = Color::new(color.r * 255.0, color.g * 255.0, color.b * 255.0, color.a * 255.0);
    _rect = render_slider("Red  ", _rect, theme, &mut color.r, 0.0, 1.0);
    _rect = render_slider("Green", _rect, theme, &mut color.g, 0.0, 1.0);
    _rect = render_slider("Blue ", _rect, theme, &mut color.b, 0.0, 1.0);
    _rect = render_slider("Alpha", _rect, theme, &mut color.a, 0.0, 1.0);

    // color = Color::new(color.r /255.0, color.g/255.0, color.b /255.0, color.a /255.0);
    let (_rect, clicked) = render_button_dev("Reset color", below_left(_rect), theme);
    (_rect, clicked)
}

pub fn as_hex(color: Color) -> String {
    let [r, g, b, a]: [u8; 4] = color.into();
    format!("0x {:0>2X} {:0>2X} {:0>2X} {:0>2X}", r, g, b, a)
}
pub fn as_hex_no_space(color: Color) -> String {
    let [r, g, b, a]: [u8; 4] = color.into();
    format!("0x{:0>2X}{:0>2X}{:0>2X}{:0>2X}", r, g, b, a)
}

pub fn palette_to_code(theme: &Theme) -> AnyResult<String> {
    let mut message: Vec<u8> = Vec::new();
    write!(
        message,
        "impl Default for Palette {{
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
    Ok("pub fn new_coloring() -> Coloring {
    Coloring {
        at_rest: StateStyle {
            bg_color: from_hex(0x190e34),
            text_color: from_hex(0xfafbf9),
            border_color: from_hex(0xfafbf9),
        },
        ..Default::default()
    }
}"
    .to_string())
}
