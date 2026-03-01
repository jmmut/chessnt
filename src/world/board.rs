use crate::coord::Coord;
use crate::render::{
    floor_corners, mesh_coord, mesh_cursor, mesh_figure_texture, mesh_progress_bar,
    mesh_texture_quad, mesh_triangle, mesh_vertical_texture,
};
use crate::theme::{color_average, color_average_weight, margin, Theme};
use crate::ui::render_text_font;
use crate::world::moves::{moves_to_string, possible_moves, Move};
use crate::world::piece::Piece;
use crate::world::referee::Referee;
use crate::world::team::Team;
use crate::TRANSPARENT;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use macroquad::color::{Color, BLUE, DARKBLUE, GRAY, GREEN, PURPLE, RED, WHITE, YELLOW};
use macroquad::math::{vec2, vec3, Rect, Vec2, Vec3};
use macroquad::models::{draw_mesh, Mesh};

// const SELECTION: Color = color_average(DARKBLUE, TRANSPARENT);
const SELECTION: Color = color_average(BLUE, GRAY);
// const RADAR: Color = color_average(color_average(RED, LIGHTGRAY), TRANSPARENT);
pub const RADAR: Color = color_average(RED, TRANSPARENT);
// const GHOST: Color = color_average(DARKPURPLE, TRANSPARENT);
const GHOST: Color = color_average(PURPLE, GRAY);
// const CURSOR: Color = color_average(DARKGREEN, TRANSPARENT);
const CURSOR_WHITE: Color = color_average_weight(color_average(GREEN, GRAY), YELLOW, 0.3);
const CURSOR_BLACK: Color = color_average_weight(color_average(GREEN, GRAY), DARKBLUE, 0.3);
// const FIGURE: Color = color_average(PINK, TRANSPARENT);
const CURSOR_HEIGHT: f32 = 0.1;
const SELECTION_HEIGHT: f32 = CURSOR_HEIGHT * 0.5;
const RADAR_HEIGHT: f32 = SELECTION_HEIGHT * 0.7;
const FLOOR_PIECE_HEIGHT: f32 = RADAR_HEIGHT * 0.2;

pub struct Board {
    cursor_white: Coord,
    cursor_black: Coord,
    selected_white: Option<usize>,
    selected_black: Option<usize>,
    size: Coord,
    pieces: Vec<Piece>,
    pub referee: Referee,
    pub piece_size: Vec2,
}

impl Board {
    pub fn new(cursor_white: Coord, cursor_black: Coord, size: Coord, pieces: Vec<Piece>) -> Self {
        Self {
            cursor_white,
            cursor_black,
            selected_white: None,
            selected_black: None,
            size,
            pieces,
            piece_size: vec2(0.3, 1.0),
            referee: Referee::new(),
        }
    }
    pub fn new_chess(cursor_white: Coord, cursor_black: Coord, size: Coord) -> Self {
        let back_column = vec![
            (0, Move::Rook),
            (1, Move::Knight),
            (2, Move::Bishop),
            (3, Move::Queen),
            (4, Move::King),
            (5, Move::Bishop),
            (6, Move::Knight),
            (7, Move::Rook),
        ];
        let mut pieces = Vec::new();
        for (row, movement) in &back_column {
            pieces.push(Piece::new(Coord::new_i(7, *row), *movement, Team::White));
            pieces.push(Piece::new(Coord::new_i(0, *row), *movement, Team::Black));
            pieces.push(Piece::new(Coord::new_i(6, *row), Move::Pawn, Team::White));
            pieces.push(Piece::new(Coord::new_i(1, *row), Move::Pawn, Team::Black));
        }

        Self::new(cursor_white, cursor_black, size, pieces)
    }
    pub fn reset(&mut self) {
        *self = Self::new_chess(self.cursor_white, self.cursor_black, self.size);
    }
    pub fn tick(&mut self, delta_s: f64) {
        self.referee.tick(delta_s, &self.pieces);
        for piece in &mut self.pieces {
            piece.tick(delta_s);
        }
    }
    fn get_selected_piece(&self, team: Team) -> Option<&Piece> {
        if let Some(index) = self.selected(team) {
            self.pieces.get(index)
        } else {
            None
        }
    }
    // fn get_selected_piece_and_pos(&self) -> Option<(&Piece, Coord)> {
    //     if let Some(index) = self.selected {
    //         Some((self.pieces.get(index).unwrap(), initial_pos))
    //     } else {
    //         None
    //     }
    // }
    fn get_selected_piece_mut(&mut self, team: Team) -> Option<&mut Piece> {
        if let Some(index) = self.selected(team) {
            self.pieces.get_mut(index)
        } else {
            None
        }
    }
    pub fn move_cursor_rel(&mut self, delta: Coord, team: Team) {
        if let Some(piece) = self.get_selected_piece_mut(team) {
            piece.pos += delta;
            piece.moved = true;
        }
        *self.cursor_mut(team) += delta;
    }
    pub fn select(&mut self, team: Team) {
        let new_selection = self.cursor(team).round();
        if let Some(_index) = self.selected(team) {
            panic!("can't select if there's something already selected");
            // TODO: swap pieces?
            // let
            //
            // for piece in &mut self.pieces {
            //     if piece.pos == old_selection {
            //         piece.pos = new_selection;
            //         self.selected = None;
            //         return;
            //     }
            // }
            // self.selected = Some(new_selection)
        } else {
            for (i, piece) in self.pieces.iter().enumerate() {
                if piece.pos == new_selection && piece.team == team {
                    if piece.cooldown_s.is_none() {
                        *self.selected_mut(team) = Some(i);
                        return;
                    }
                }
            }
        }
    }
    pub fn deselect(&mut self, team: Team) {
        if let Some(selected_i) = self.selected(team) {
            self.pieces[selected_i].cooldown_s = Some(0.0);
            let initial = self.pieces[selected_i].initial_pos;
            let any_overlap_i = self.any_overlapping_piece(selected_i);
            if let Some(overlap_i) = any_overlap_i {
                let moves = possible_moves(self.size, &self.pieces, selected_i);
                let referee_saw = self.referee.saw_any_piece(&self.pieces, vec![selected_i]);
                let rounded_pos = self.pieces[overlap_i].pos.round();
                let enemy = self.pieces[overlap_i].team != self.pieces[selected_i].team;
                if enemy {
                    if moves.contains(&rounded_pos) && referee_saw {
                        self.kill(overlap_i);
                        self.pieces[selected_i].set_pos(rounded_pos);
                        self.referee.turn.toggle();
                    } else {
                        self.pieces[selected_i].set_pos(initial);
                    }
                } else {
                    let initial = self.pieces[selected_i].initial_pos;
                    self.pieces[overlap_i].set_pos(initial.round());
                    self.pieces[selected_i].set_pos(rounded_pos);
                    if self
                        .referee
                        .saw_any_piece(&self.pieces, vec![selected_i, overlap_i])
                    {
                        // TODO: defer after animation
                        self.pieces[overlap_i].set_pos(rounded_pos);
                        self.kill(selected_i);
                    }
                }
                // TODO: maybe no need for an different key press for swapping?
            } else {
                let rounded_pos = self.pieces[selected_i].pos.round(); // TODO: leave positions unrounded
                let initial_pos = self.pieces[selected_i].initial_pos.round();
                let mut moves = possible_moves(self.size, &self.pieces, selected_i);
                moves.push(self.pieces[selected_i].initial_pos);
                let referee_saw = self.referee.saw_any_piece(&self.pieces, vec![selected_i]);
                self.pieces[selected_i].set_pos(rounded_pos);
                if referee_saw {
                    if !moves.contains(&rounded_pos) {
                        self.kill(selected_i);
                    } else if self.referee.turn != self.pieces[selected_i].team {
                        self.kill(selected_i);
                    } else if rounded_pos != initial_pos {
                        self.referee.turn.toggle();
                    } else {
                        // picked up the piece and dropped it the same place.
                        // TODO: punish this?
                    }
                }
            }
            *self.selected_mut(team) = None;
            *self.cursor_mut(team) = self.cursor(team).round();
        } else {
            panic!("logic error: deselecting but there was no selection");
        }
    }
    pub fn is_selected(&self, team: Team) -> bool {
        self.selected(team).is_some()
    }
    fn selected(&self, team: Team) -> Option<usize> {
        if team == Team::White {
            self.selected_white
        } else {
            self.selected_black
        }
    }
    fn selected_mut(&mut self, team: Team) -> &mut Option<usize> {
        if team == Team::White {
            &mut self.selected_white
        } else {
            &mut self.selected_black
        }
    }
    fn cursor(&self, team: Team) -> Coord {
        if team == Team::White {
            self.cursor_white
        } else {
            self.cursor_black
        }
    }
    fn cursor_mut(&mut self, team: Team) -> &mut Coord {
        if team == Team::White {
            &mut self.cursor_white
        } else {
            &mut self.cursor_black
        }
    }

    fn kill(&mut self, selected_i: usize) {
        self.pieces[selected_i].alive = false;
        self.pieces[selected_i].pos.row = -2.0;
        while self.any_overlapping_piece(selected_i).is_some() {
            self.pieces[selected_i].pos.row -= 1.0;
        }
    }

    fn any_overlapping_piece(&mut self, selected_i: usize) -> Option<usize> {
        any_overlapping_piece(selected_i, &self.pieces)
    }
}

fn any_overlapping_piece(selected_i: usize, pieces: &Vec<Piece>) -> Option<usize> {
    let selected_rounded = pieces[selected_i].pos.round();
    any_other_piece_at(selected_rounded, selected_i, pieces)
}

pub fn any_other_piece_at(pos: Coord, index: usize, pieces: &Vec<Piece>) -> Option<usize> {
    for (i, piece) in pieces.iter().enumerate() {
        if i != index && piece.pos.round() == pos {
            return Some(i);
        }
    }
    None
}

impl Board {
    /// assumes 3d camera is enabled
    pub fn draw_world(&self, theme: &Theme) {
        let mut meshes = Vec::new();
        self.draw_floor(theme);

        meshes.extend(self.selection_meshes(Team::White));
        meshes.extend(self.selection_meshes(Team::Black));
        meshes.extend(self.piece_meshes(theme));
        meshes.extend(self.referee_meshes(theme));
        meshes.extend(self.possible_moves_meshes(Team::White));
        meshes.extend(self.possible_moves_meshes(Team::Black));

        meshes.sort_by(|a, b| depth(a).total_cmp(&depth(b)));
        for mesh in meshes {
            draw_mesh(&mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }
    }

    /// assumes default camera is enabled
    pub fn draw_ui(&self, theme: &Theme) {
        let _rect = theme.screen_rect();
        let _rect = Anchor::inside(
            _rect,
            Layout::Vertical {
                direction: Vertical::Bottom,
                alignment: Horizontal::Left,
            },
            margin(theme),
        )
        .get_rect(vec2(0.0, 0.0));
        let _rect = self.draw_turn(_rect, theme);
        let _rect = self.draw_piece_info(_rect, Team::White, theme);
        let _rect = self.draw_piece_info(_rect, Team::Black, theme);
    }

    fn draw_piece_info(&self, previous_rect: Rect, team: Team, theme: &Theme) -> Rect {
        fn team_name(team: Team) -> &'static str {
            if team.is_white() {
                "WHITE"
            } else {
                "BLACK"
            }
        }
        for piece in &self.pieces {
            if piece.pos.round() == self.cursor(team).round() {
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

    fn selection_meshes(&self, team: Team) -> Vec<Mesh> {
        if let Some(_selected) = self.get_selected_piece(team) {
            // meshes.extend(mesh_cursor(_selected.pos, SELECTION, SELECTION_HEIGHT));
            vec![]
        } else {
            mesh_cursor(self.cursor(team), cursor_color(team), CURSOR_HEIGHT)
        }
    }

    fn piece_meshes(&self, theme: &Theme) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for piece in &self.pieces {
            meshes.push(mesh_figure_texture(
                piece,
                if piece.team.is_white() { WHITE } else { GRAY },
                theme.textures.placeholder,
                self.piece_size,
            ));
            // meshes.push(to_mesh(
            //     floor_corners(piece.pos + Coord::new_f(0.5, 0.5), FLOOR_PIECE_HEIGHT * 1.1, 0.2),
            //     BLUE,
            // ));

            meshes.extend(mesh_progress_bar(
                piece.pos,
                self.piece_size,
                piece.cooldown_progress(),
            ));

            meshes.push(mesh_texture_quad(
                floor_corners(piece.pos.round(), FLOOR_PIECE_HEIGHT, 1.0),
                WHITE,
                Some(theme.textures.pieces[&(piece.team, piece.moveset[0])]),
                piece.team.is_white(),
                true,
            ));

            // meshes.push(render_text_3d(
            //     &moves_to_string(&piece.moveset),
            //     Anchor::bottom_left(piece.pos.column, 2.0),
            //     piece.pos.row,
            //     theme,
            // ));
        }
        meshes
    }

    fn referee_meshes(&self, theme: &Theme) -> Vec<Mesh> {
        let coord_00 = self.referee.pos_v3(self.piece_size.x, 0.0);
        let looking_leftwards = self.referee.looking_leftwards();
        let mesh = mesh_vertical_texture(
            coord_00,
            WHITE,
            Some(theme.textures.placeholder),
            looking_leftwards,
            self.piece_size,
        );
        let mut meshes = vec![mesh];

        let bar = mesh_progress_bar(
            self.referee.pos_c(),
            self.piece_size,
            self.referee.focus_progress(),
        );
        meshes.extend(bar);

        let [radar_base, radar_right, radar_left] = self.referee.radar();
        let square_offset = vec3(0.5, RADAR_HEIGHT, 0.5);
        let radar_base = radar_base.into::<Vec3>() + square_offset;
        let radar_right = radar_right.into::<Vec3>() + square_offset;
        let radar_left = radar_left.into::<Vec3>() + square_offset;
        let radar = mesh_triangle([radar_base, radar_right, radar_left], RADAR);
        if self.referee.render_radar {
            meshes.push(radar);
        }
        meshes
    }

    fn possible_moves_meshes(&self, team: Team) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        if let Some(index) = self.selected(team) {
            meshes.extend(mesh_cursor(
                self.pieces[index].initial_pos,
                GHOST,
                SELECTION_HEIGHT,
            ));
            for movement in possible_moves(self.size, &self.pieces, index) {
                meshes.extend(mesh_cursor(movement, SELECTION, SELECTION_HEIGHT))
            }
        }
        meshes
    }

    fn draw_floor(&self, theme: &Theme) {
        for column in 0..self.size.column() {
            for row in 0..self.size.row() {
                let color = if (row + column) % 2 == 0 {
                    theme.palette.black_cells
                } else {
                    theme.palette.white_cells
                };
                draw_mesh(&mesh_coord(Coord::new_i(column, row), color));
            }
        }
    }
}

pub fn cursor_color(team: Team) -> Color {
    if team.is_white() {
        CURSOR_WHITE
    } else {
        CURSOR_BLACK
    }
}

/// assumes meshes are just quads, with vertices in zig-zag order. (top left, top right, bottom left, bottom right).
fn depth(mesh: &Mesh) -> f32 {
    (mesh.vertices[0].position.z + mesh.vertices[2].position.z) * 0.5 * 0.001
        + (mesh.vertices[0].position.y + mesh.vertices[2].position.y) * 0.5 * 10.0
}
