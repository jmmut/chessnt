use crate::screen::anchorer::{inside_initial, inside_initial_pad};
use crate::screen::theme::{margin, Theme};
use crate::screen::ui::{measure_button, measure_title, render_text_font, render_title};
use crate::world::board::Board;
use crate::world::moves::moves_to_string;
use crate::world::team::Team;
use juquad::draw::{draw_rect, draw_rect_lines};
use juquad::lazy::add_contour;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::Widget;
use macroquad::math::{Rect, Vec2};

impl Board {
    /// assumes default camera is enabled
    pub fn draw_ui(&self, theme: &Theme) {
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
            draw_game_finished(won, theme);
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
fn draw_game_finished(won: Team, theme: &Theme) -> bool {
    let layout = Layout::vertical(Vertical::Bottom, Horizontal::Center);
    let initial = inside_initial_pad(theme.screen_rect(), layout, theme.screen * 0.1);

    let anchor = Anchor::below(initial, Horizontal::Center, 0.0);
    let text = measure_title(&format!("{} won!", won), theme, anchor);

    let anchor = Anchor::below(text.rect(), Horizontal::Center, margin(theme).y);
    let mut button = measure_button("Restart", theme, anchor);

    let panel_rect = add_contour(text.rect().combine_with(button.rect()), margin(theme));

    let thickness = 8.0;
    draw_rect_lines(
        add_contour(panel_rect, Vec2::splat(thickness * 0.5)),
        thickness,
        theme.coloring().button_coloring.hovered.border_color,
    );
    draw_rect(panel_rect, theme.coloring().text_coloring.bg_color);

    draw_rect(text.rect(), theme.coloring().text_coloring.bg_color);
    text.render_default(&theme.coloring().text_coloring);

    let interaction = button.interact();
    button.render_default(&theme.button_coloring());
    interaction.is_clicked()
}
