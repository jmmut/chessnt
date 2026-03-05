use crate::screen::anchorer::inside_initial;
use crate::screen::theme::Theme;
use crate::screen::ui::{render_text_font, render_title};
use crate::world::board::Board;
use crate::world::moves::moves_to_string;
use crate::world::team::Team;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use macroquad::math::Rect;

impl Board {
    /// assumes default camera is enabled
    pub fn draw_ui(&self, theme: &Theme) {
        let screen = theme.screen_rect();
        let layout = Layout::vertical(Vertical::Bottom, Horizontal::Left);
        let _rect = inside_initial(theme, screen, layout);
        let _rect = self.draw_turn(_rect, theme);
        let _rect = self.draw_piece_info(_rect, Team::White, theme);
        let _rect = self.draw_piece_info(_rect, Team::Black, theme);

        let layout = Layout::vertical(Vertical::Bottom, Horizontal::Center);
        let mut _rect = inside_initial(theme, screen, layout);
        for team in self.in_check() {
            _rect = self.draw_check(_rect, team, theme);
        }
    }

    fn draw_piece_info(&self, previous_rect: Rect, team: Team, theme: &Theme) -> Rect {
        fn team_name(team: Team) -> &'static str {
            if team.is_white() {
                "WHITE"
            } else {
                "BLACK"
            }
        }
        for piece in self.pieces() {
            if piece.pos_i() == self.cursor(team).round() {
                return render_text_font(
                    &format!(
                        "{}: {} {}",
                        team_name(team),
                        team_name(piece.team),
                        moves_to_string(&piece.moveset).to_uppercase()
                    ),
                    Anchor::below(previous_rect, Horizontal::Left, 0.0),
                    theme,
                    theme.font_title(),
                );
            }
        }
        previous_rect
    }
    fn draw_turn(&self, previous_rect: Rect, theme: &Theme) -> Rect {
        render_text_font(
            &format!(
                "It's {}'s turn",
                if self.referee.turn.is_white() {
                    "WHITE"
                } else {
                    "BLACK"
                },
            ),
            Anchor::below(previous_rect, Horizontal::Left, 0.0),
            theme,
            theme.font_title(),
        )
    }

    fn draw_check(&self, previous_rect: Rect, team: Team, theme: &Theme) -> Rect {
        let team_name = if team.is_white() { "WHITE" } else { "BLACK" };
        render_title(
            &format!("{} king is in check!", team_name),
            Anchor::below(previous_rect, Horizontal::Center, 0.0),
            theme,
        )
    }
}
