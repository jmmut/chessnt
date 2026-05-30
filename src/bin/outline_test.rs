use chessnt::AnyResult;
use chessnt::screen::shader::names::{OUTLINE_COLOR, OUTLINE_THICKNESS, SCREEN, TEXT_COLOR};
use chessnt::screen::shader::{OUTLINE_FRAGMENT_SHADER, OUTLINE_VERTEX_SHADER, outline_shader};
use chessnt::screen::theme::{AllColoring, Fonts};
use chessnt::screen::ui::{below_left, format_slider_text, render_slider_raw};
use macroquad::prelude::*;

#[macroquad::main("outline")]
async fn main() -> AnyResult<()> {
    let render_target_filter_mode = FilterMode::Nearest;
    let outline = outline_shader(OUTLINE_VERTEX_SHADER, OUTLINE_FRAGMENT_SHADER)?;
    let mut render_target = render_target_msaa(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(render_target_filter_mode);
    let fonts = Fonts {
        titles: load_ttf_font("assets/fonts/LilitaOne-Regular.ttf").await?,
        text: load_ttf_font("assets/fonts/TitilliumWeb-SemiBold.ttf").await?,
        dev: load_ttf_font("assets/fonts/JetBrainsMono-Medium.ttf").await?,
    };
    let fonts_list = [
        (&fonts.titles, "title"),
        (&fonts.text, "regular"),
        (&fonts.dev, "dev"),
    ];
    let coloring = AllColoring::default();
    let mut font_size = 32.0;
    let mut use_material = true;
    let mut direct_to_screen = false;
    let mut thickness: f32 = 2.0;
    let text_color = Color::new(0.8, 0.8, 0.9, 1.0);
    let outline_color = Color::new(0.3, 0.0, 0.0, 1.0);
    let background_color = Color::new(0.4, 0.6, 0.5, 1.0);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::M) {
            use_material = !use_material;
        }
        if is_key_pressed(KeyCode::N) {
            thickness = (thickness - 1.0).clamp(0.0, 50.0);
        }
        if is_key_pressed(KeyCode::B) {
            thickness = (thickness + 1.0).clamp(0.0, 50.0);
        }
        if is_key_pressed(KeyCode::S) {
            direct_to_screen = !direct_to_screen;
        }

        let screen = vec2(screen_width(), screen_height());

        if direct_to_screen {
            set_default_camera();
            clear_background(background_color);
            draw_all_text(
                fonts_list,
                font_size,
                text_color,
                screen,
                direct_to_screen,
                use_material,
            );
        } else {
            if screen != render_target.texture.size() {
                render_target = macroquad::prelude::render_target(
                    screen_width() as u32,
                    screen_height() as u32,
                );
                render_target.texture.set_filter(render_target_filter_mode);
            }

            set_camera(&Camera2D {
                zoom: 2.0 / screen,
                target: screen * 0.5,
                render_target: Some(render_target.clone()),
                ..Default::default()
            });
            clear_background(background_color.with_alpha(0.0));
            draw_all_text(
                fonts_list,
                font_size,
                text_color,
                screen,
                direct_to_screen,
                use_material,
            );

            set_default_camera();
            clear_background(background_color);
            if use_material {
                gl_use_material(&outline);
                outline.set_uniform(SCREEN, screen);
                outline.set_uniform(OUTLINE_THICKNESS, thickness);
                outline.set_uniform(TEXT_COLOR, text_color);
                outline.set_uniform(OUTLINE_COLOR, outline_color);
            }

            draw_texture_ex(
                &render_target.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(screen),
                    ..Default::default()
                },
            );
            if use_material {
                gl_use_default_material();
            }
        }
        let mut rect = Rect::default();
        render_slider_raw(
            &format_slider_text("font size", font_size),
            5.0,
            50.0,
            &mut font_size,
            &coloring,
            &fonts.dev,
            16.0,
            below_left,
            &mut rect,
        );
        font_size = font_size.round();
        next_frame().await;
    }
    Ok(())
}

fn draw_all_text(
    fonts_list: [(&Font, &str); 3],
    font_size: f32,
    text_color: Color,
    screen: Vec2,
    direct_to_screen: bool,
    use_material: bool,
) {
    let shader = if direct_to_screen {
        "".to_string()
    } else {
        format!(", shader {}", on_or_off(use_material))
    };
    let text = format!(
        "default font, render target {}{}",
        on_or_off(!direct_to_screen),
        shader
    );
    draw_text(
        &text,
        screen.x * 0.05,
        screen.y * 0.2,
        font_size,
        text_color,
    );
    for (i, (font, _name)) in fonts_list.iter().enumerate() {
        draw_text_ex(
            &text,
            screen.x * 0.05,
            screen.y * (0.4 + 0.2 * i as f32),
            TextParams {
                font: Some(font),
                font_size: font_size as u16,
                color: text_color,
                ..Default::default()
            },
        );
    }
}

fn on_or_off(value: bool) -> &'static str {
    if value { "on" } else { "off" }
}
