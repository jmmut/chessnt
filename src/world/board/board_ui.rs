use crate::screen::anchorer::inside_initial;
use crate::screen::theme::Theme;
use crate::screen::ui::{render_text_font, render_title};
use crate::world::board::Board;
use crate::world::moves::moves_to_string;
use crate::world::team::Team;
use juquad::elm::button::Button;
use juquad::elm::container::Container;
use juquad::elm::text::Text;
use juquad::elm::widget::compute_layout;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use macroquad::math::Rect;

#[derive(Copy, Clone)]
pub enum Message {
    Exit,
    Restart,
    ReloadTextures,
    ToggleBot(Team),
    ToggleRadar,
    ToggleReferee,
    TargetFPS(Option<f64>),
    ReloadShaderCharacter,
    ToggleSinCity,
    Zoom(bool),
}

impl Board {
    /// assumes default camera is enabled
    pub fn draw_ui(&self, theme: &Theme) -> Vec<Message> {
        let screen = theme.screen_rect();
        let layout_center = Layout::vertical(Vertical::Bottom, Horizontal::Center);
        let _rect = inside_initial(theme, screen, layout_center);
        let _rect = self.draw_turn(_rect, theme);
        // let _rect = self.draw_piece_info(_rect, Team::White, theme);
        // let _rect = self.draw_piece_info(_rect, Team::Black, theme);
        for (team, _kind_index) in self.in_check() {
            let corner = if team.is_white() {
                Horizontal::Right
            } else {
                Horizontal::Left
            };
            let layout = Layout::vertical(Vertical::Bottom, corner);
            let rect = inside_initial(theme, screen, layout);
            let anchor = Anchor::below(rect, corner, 0.0);
            self.draw_check(anchor, team, theme);
        }
        if let Some(won) = self.winning_team {
            draw_game_finished(won, theme)
        } else {
            Vec::new()
        }
    }

    #[allow(unused)]
    fn draw_piece_info(&self, previous_rect: Rect, team: Team, theme: &Theme) -> Rect {
        for piece in self.pieces() {
            if piece.pos_i() == self.cursor(team).round() {
                return render_text_font(
                    &format!(
                        "{}: {} {}",
                        team,
                        piece.team,
                        moves_to_string(&piece.moveset).to_uppercase()
                    ),
                    theme,
                    theme.font_title(),
                    Anchor::below(previous_rect, Horizontal::Left, 0.0),
                );
            }
        }
        previous_rect
    }
    fn draw_turn(&self, previous_rect: Rect, theme: &Theme) -> Rect {
        render_text_font(
            &format!(
                "{}",
                if self.referee.turn.is_white() {
                    "WHITE"
                } else {
                    "BLACK"
                },
            ),
            theme,
            theme.font_title(),
            Anchor::below(previous_rect, Horizontal::Center, 0.0),
        )
    }

    fn draw_check(&self, anchor: Anchor, _team: Team, theme: &Theme) -> Rect {
        render_title("Check!", theme, anchor)
    }
}

/// returns if the user clicked "Restart"
fn draw_game_finished(won: Team, theme: &Theme) -> Vec<Message> {
    let layout = Layout::vertical(Vertical::Bottom, Horizontal::Center);
    let container_style = theme.container_style();
    let title_style = theme.title_style();
    let button_style = theme.button_style();

    let mut ui = Container::new(
        container_style,
        vec![
            Text::new(title_style, format!("{} won!", won)),
            Button::new_text(button_style, Message::Restart, "Restart"),
        ],
    );
    let mut panel = theme.screen_rect();
    panel.y = panel.h * 0.3;
    compute_layout(&mut *ui, panel, layout);
    let messages = ui.interact();
    ui.render();
    messages
}
