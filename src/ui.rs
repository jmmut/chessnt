use crate::board::Board;
use crate::theme::{CameraPos, Theme};
use crate::time::Time;
use juquad::draw::draw_rect;
use juquad::input::input_macroquad::InputMacroquad;
use juquad::lazy::{Interactable, Renderable, Style, WidgetTrait};
use juquad::widgets::anchor::{Anchor, Horizontal, Vertical};
use juquad::widgets::button::Button;
use juquad::widgets::text::TextRect;
use juquad::widgets::{Interaction, StateStyle, Widget};
use macroquad::math::Rect;
use macroquad::prelude::{Font, TextParams};

pub fn render_text(text: &str, anchor: Anchor, theme: &Theme) -> Rect {
    render_text_font(text, anchor, theme, theme.font())
}
pub fn render_text_font(text: &str, anchor: Anchor, theme: &Theme, font: Font) -> Rect {
    let t = TextRect::new_generic(
        text,
        anchor,
        theme.font_size(),
        Some(font),
        macroquad::prelude::measure_text,
    );
    draw_rect(t.rect(), theme.coloring().at_rest.bg_color);
    t.render_default(&theme.coloring().at_rest);
    t.rect()
}
pub fn render_button(text: &str, anchor: Anchor, theme: &Theme) -> (Rect, Interaction) {
    render_button_font(text, anchor, theme, theme.font())
}
pub fn render_button_font(
    text: &str,
    anchor: Anchor,
    theme: &Theme,
    font: Font,
) -> (Rect, Interaction) {
    let mut t = Button::new_generic(
        text,
        anchor,
        theme.font_size(),
        Some(font),
        macroquad::prelude::measure_text,
        Box::new(InputMacroquad),
    );
    let interaction = t.interact();
    t.render_default(&theme.coloring());
    (t.rect(), interaction)
}
pub static mut SCALE: f32 = 13.78;
/*
pub fn render_text_3d(text: &str, anchor: Anchor, z: f32, theme: &Theme) -> Mesh {
    panic!("doesn't work! this creates a memory leak");
    push_camera_state();

    let t = TextRect::new_generic(
        text,
        anchor,
        theme.font_size(),
        Some(theme.font()),
        macroquad::prelude::measure_text,
    );

    let test_text = "FFFF";
    let mut m = measure_text(test_text, None, theme.font_size() as u16, 1.0);
    m.width = 20.0 * unsafe { SCALE };
    m.height = 10.0 * unsafe { SCALE };
    // let render_target = macroquad::prelude::render_target(m.width as u32, m.height as u32);
    let render_target = macroquad::prelude::render_target(
        (t.rect().w * unsafe { SCALE }) as u32,
        (t.rect().h * unsafe { SCALE }) as u32,
    );
    // render_target.texture.set_filter(FilterMode::Nearest);
    set_camera(&Camera2D {
        //     target: vec2(sw * 0.5, sh * 0.5),
        // target: vec2(m.width, m.height),
        // target: vec2(0.0, 0.0),
        target: t.rect().size(),
        zoom: vec2(1.0, 1.0) / t.rect().size() / unsafe { SCALE } * 0.005,
        // zoom: vec2(1.0 / m.width, 1.0 / m.height) / Vec2::splat(unsafe { SCALE } * 0.005),
        // zoom: vec2(1.0 / m.width, 1.0/ m.height) / Vec2::splat(0.76) * 2.0,
        render_target: Some(render_target),
        ..Default::default()
    });
    clear_background(GRAY);
    // draw_rect(t.rect(), theme.coloring().at_rest.bg_color);
    // t.render(&theme.coloring().at_rest, draw_text);
    macroquad::text::draw_text(
        test_text,
        0.0,
        theme.font_size() * 0.5,
        theme.font_size(),
        BLACK,
    );
    pop_camera_state();

    let rect = anchor.get_rect(t.rect().size());
    let pos = Coord::new_f(rect.x, z + 0.5).to_vec3(rect.bottom());
    mesh_vertical_texture(pos, 0.5, WHITE, Some(render_target.texture), false)
    // mesh_vertical_texture(pos, 0.5, WHITE, None)
}
*/
pub fn below_left(rect: Rect) -> Anchor {
    Anchor::below(rect, Horizontal::Left, 0.0)
}
pub fn rightwards(rect: Rect) -> Anchor {
    Anchor::rightwards(rect, Vertical::Center, 0.0)
}
pub fn draw_text(
    text: &str,
    x: f32,
    y: f32,
    font_size: f32,
    style: &StateStyle,
    font: Option<Font>,
) {
    let params = TextParams {
        font: font.unwrap_or(Font::default()),
        font_size: font_size as u16,
        color: style.text_color,
        font_scale: 1.0,
        ..TextParams::default()
    };
    macroquad::text::draw_text_ex(text, x, y, params)
}

#[derive(Eq, PartialEq)]
pub enum DevUiMenu {
    Hidden,
    Main,
    Camera,
    Referee,
}

pub struct DevUi {
    pub menu: DevUiMenu,
}

impl DevUi {
    pub fn new() -> Self {
        Self {
            menu: DevUiMenu::Main,
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
    ) {
        match self.menu {
            DevUiMenu::Hidden => {}
            DevUiMenu::Main => self.draw_main(theme),
            DevUiMenu::Camera => self.draw_camera(time, theme, board, camera),
            DevUiMenu::Referee => self.draw_referee(theme, board),
        }
    }
    fn dev_ui_title(theme: &mut Theme) -> Rect {
        let _rect = render_text(
            "DEV UI (toggle with '/')",
            Anchor::top_left(0.0, 0.0),
            theme,
        );
        _rect
    }

    fn navigation(&mut self, theme: &mut Theme, _rect: Rect, text: &str, menu: DevUiMenu) -> Rect {
        let (rect, back_clicked) = render_button(text, below_left(_rect), theme);
        if back_clicked.is_clicked() {
            self.menu = menu;
        }
        rect
    }

    fn draw_main(&mut self, theme: &mut Theme) {
        let _rect = Self::dev_ui_title(theme);
        let _rect = self.navigation(theme, _rect, "Camera controls", DevUiMenu::Camera);
        let _rect = self.navigation(theme, _rect, "Inspect referee", DevUiMenu::Referee);
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
        let _rect = render_text(&text, below_left(_rect), theme);
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
        self.navigation(theme, _rect, "Back", DevUiMenu::Main);
    }

    fn draw_referee(&mut self, theme: &mut Theme, board: &mut Board) {
        let _rect = render_text(
            "DEV UI (toggle with '/')",
            Anchor::top_left(0.0, 0.0),
            theme,
        );
        let dir = board.referee.dir_c();
        let _rect = render_text(
            &format!("Referee dir: {:0>5.2} {:0>5.2}", dir.column, dir.row),
            below_left(_rect),
            theme,
        );
        let _rect = render_text(
            &format!("trip time: {:0>5.2}", board.referee.trip_time),
            below_left(_rect),
            theme,
        );
        self.navigation(theme, _rect, "Back", DevUiMenu::Main);
    }
}

fn render_slider(
    text: &str,
    rect: Rect,
    theme: &Theme,
    value: &mut f32,
    min: f32,
    max: f32,
) -> Rect {
    let new_rect = render_text(
        &format!("{}: {:0>5.2}", text, value),
        below_left(rect),
        theme,
    );
    let mut slider = juquad::lazy::slider::Slider::new(Style::default(), min, max, *value);
    slider.set_pos(rightwards(new_rect).get_top_left_pixel(slider.size()));
    *value = *(slider
        .interact()
        .into_iter()
        .next()
        .unwrap()
        .downcast::<f32>()
        .unwrap());
    slider.render_interactive(Interaction::None);
    new_rect
}
