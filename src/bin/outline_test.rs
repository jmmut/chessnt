use chessnt::AnyResult;
use chessnt::screen::shader::names::{OUTLINE_COLOR, OUTLINE_THICKNESS, SCREEN, TEXT_COLOR};
use chessnt::screen::shader::{OUTLINE_FRAGMENT_SHADER, OUTLINE_VERTEX_SHADER, outline_shader};
use chessnt::screen::theme::Fonts;
use macroquad::prelude::*;

#[macroquad::main("outline")]
async fn main() -> AnyResult<()> {
    let outline = outline_shader(OUTLINE_VERTEX_SHADER, OUTLINE_FRAGMENT_SHADER)?;
    let mut render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
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
    let font_size = 16.0;
    let mut use_material = true;
    let mut direct_to_screen = false;
    let mut thickness: f32 = 2.0;
    let text_color = Color::new(0.8, 0.8, 0.9, 1.0);
    let outline_color = Color::new(0.3, 0.0, 0.0, 1.0);
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
            clear_background(Color::new(0.5, 0.7, 0.6, 1.0));
            draw_all_text(fonts_list, font_size, &mut thickness, text_color, screen);
        } else {
            if screen != render_target.texture.size() {
                render_target = macroquad::prelude::render_target(
                    screen_width() as u32,
                    screen_height() as u32,
                );
                render_target.texture.set_filter(FilterMode::Nearest);
            }

            set_camera(&Camera2D {
                zoom: 2.0 / screen,
                target: screen * 0.5,
                render_target: Some(render_target.clone()),
                ..Default::default()
            });
            clear_background(Color::new(0.0, 0.0, 0.0, 0.0));
            draw_all_text(fonts_list, font_size, &mut thickness, text_color, screen);

            set_default_camera();
            clear_background(Color::new(0.5, 0.7, 0.6, 1.0));
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
        next_frame().await;
    }
    Ok(())
}

fn draw_all_text(
    fonts_list: [(&Font, &str); 3],
    font_size: f32,
    thickness: &mut f32,
    text_color: Color,
    screen: Vec2,
) {
    draw_text(
        "Some text with default font",
        screen.x * 0.05,
        screen.y * 0.2,
        font_size,
        text_color,
    );
    for (i, (font, name)) in fonts_list.iter().enumerate() {
        draw_text_ex(
            &format!("Some {} text, outline thickness {}", name, thickness),
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
