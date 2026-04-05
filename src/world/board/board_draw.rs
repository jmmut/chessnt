use crate::TRANSPARENT;
use crate::core::coord::{Coord, ICoord};
use crate::screen::render::{
    floor_corners, horizontal_quad, mesh_cursor, mesh_cursor_width, mesh_figure_texture,
    mesh_progress_bar, mesh_quad, mesh_texture_quad, mesh_triangle, mesh_vertical_texture, quad,
};
use crate::screen::shader::names::{
    COLOR_BLACK, COLOR_WHITE, POSITION_X_NAME, POSITION_Y_NAME, RADAR, REFEREE_SAW, SIN_CITY, TEAM,
    TILES,
};
use crate::screen::theme::Theme;
use crate::world::board::{Board, other_pieces_at};
use crate::world::moves::possible_moves;
use crate::world::piece::Piece;
use crate::world::team::Team;
use macroquad::color::{Color, WHITE};
use macroquad::input::mouse_position;
use macroquad::material::{gl_use_default_material, gl_use_material};
use macroquad::math::{Vec2, Vec3, vec3};
use macroquad::models::{Mesh, draw_mesh};
use macroquad::prelude::{screen_height, screen_width};

const CURSOR_HEIGHT: f32 = 0.1;
const SELECTION_HEIGHT: f32 = CURSOR_HEIGHT * 0.5;
const RADAR_HEIGHT: f32 = -SELECTION_HEIGHT * 0.7;
const FLOOR_PIECE_HEIGHT: f32 = -RADAR_HEIGHT * 0.2;

impl Board {
    /// assumes 3d camera is enabled
    pub fn draw_world(&self, theme: &Theme) {
        let mut meshes = Vec::new();
        meshes.extend(self.referee_meshes(theme));
        for mesh in &meshes {
            draw_mesh(mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }
        meshes.clear();
        self.draw_floor(theme);

        meshes.extend(self.selection_meshes(Team::White, theme));
        meshes.extend(self.selection_meshes(Team::Black, theme));
        meshes.extend(self.possible_moves_meshes(Team::White, theme));
        meshes.extend(self.possible_moves_meshes(Team::Black, theme));
        meshes.extend(self.checks_meshes(theme));
        meshes.sort_by(|a, b| depth(a).total_cmp(&depth(b)));
        for mesh in &meshes {
            draw_mesh(&mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }
        meshes.clear();
        self.draw_piece_meshes(theme);

        meshes.extend(self.turn_light_meshes(theme));

        for mesh in meshes {
            draw_mesh(&mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }
    }

    fn selection_meshes(&self, team: Team, theme: &Theme) -> Vec<Mesh> {
        if let Some(_selected) = self.get_selected_piece(team) {
            // meshes.extend(mesh_cursor(_selected.pos, SELECTION, SELECTION_HEIGHT));
            vec![]
        } else {
            mesh_cursor(self.cursor(team), cursor_color(team, theme), CURSOR_HEIGHT)
        }
    }

    fn draw_piece_meshes(&self, theme: &Theme) {
        let mut meshes = Vec::new();
        let mut character_meshes = Vec::new();
        for (i, piece) in self.pieces.iter().enumerate() {
            character_meshes.push((
                i,
                mesh_figure_texture(
                    piece,
                    if piece.team.is_white() {
                        theme.palette.mask_white
                    } else {
                        theme.palette.mask_black
                    },
                    theme.textures.placeholder.clone(),
                    self.piece_size,
                ),
            ));
            // meshes.push(to_mesh(
            //     floor_corners(piece.pos + Coord::new_f(0.5, 0.5), FLOOR_PIECE_HEIGHT * 1.1, 0.2),
            //     BLUE,
            // ));

            meshes.extend(mesh_progress_bar(
                piece.pos_f(),
                self.piece_size,
                piece.cooldown_progress(),
                theme,
            ));

            meshes.push(mesh_texture_quad(
                floor_corners(piece.pos_i(), FLOOR_PIECE_HEIGHT, 1.0),
                WHITE,
                Some(theme.textures.pieces[&(piece.team, piece.moveset.single())].clone()),
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

        meshes.sort_by(|a, b| depth(a).total_cmp(&depth(b)));
        for mesh in meshes {
            draw_mesh(&mesh); // can't render cursor and figures online because of intersecting quads with transparencies
        }

        gl_use_material(&theme.materials.character);
        character_meshes.sort_by(|a, b| depth(&a.1).total_cmp(&depth(&b.1)));
        for (i, character) in character_meshes {
            let saw = self.referee.saw_any_piece(self.pieces(), vec![i]);
            theme
                .materials
                .character
                .set_uniform(REFEREE_SAW, saw as i32);
            theme
                .materials
                .character
                .set_uniform(TEAM, !self.pieces()[i].team.is_white() as i32);
            theme
                .materials
                .character
                .set_uniform(SIN_CITY, theme.sin_city as i32);

            draw_mesh(&character);
        }
        gl_use_default_material();
    }

    fn referee_meshes(&self, theme: &Theme) -> Vec<Mesh> {
        let coord_00 = self.referee.pos_v3(self.piece_size.x, 0.0);
        let looking_leftwards = self.referee.looking_leftwards();
        let mesh = mesh_vertical_texture(
            coord_00,
            WHITE,
            Some(theme.textures.placeholder.clone()),
            looking_leftwards,
            self.piece_size,
        );
        let mut meshes = vec![mesh];

        let bar = mesh_progress_bar(
            self.referee.pos_c(),
            self.piece_size,
            self.referee.focus_progress(),
            theme,
        );
        meshes.extend(bar);

        let [radar_base, radar_right, radar_left] = self.referee.radar();
        let square_offset = vec3(0.5, RADAR_HEIGHT, 0.5);
        let radar_base = radar_base.into::<Vec3>() + square_offset;
        let radar_right = radar_right.into::<Vec3>() + square_offset;
        let radar_left = radar_left.into::<Vec3>() + square_offset;
        let radar = mesh_triangle([radar_base, radar_right, radar_left], theme.palette.radar);
        if self.referee.render_radar {
            meshes.push(radar);
        }
        meshes
    }

    fn possible_moves_meshes(&self, team: Team, theme: &Theme) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        if let Some(index) = self.selected(team) {
            meshes.extend(mesh_cursor(
                self.pieces[index].initial_pos.into(),
                theme.palette.ghost,
                SELECTION_HEIGHT,
            ));
            for movement in possible_moves(index, &self.pieces, self.size, self.ever_moved()) {
                meshes.extend(mesh_cursor(
                    movement.into(),
                    theme.palette.selection,
                    SELECTION_HEIGHT,
                ))
            }
        }
        meshes
    }

    fn draw_floor(&self, theme: &Theme) {
        gl_use_material(&theme.materials.floor);
        let position_in_pixels_tuple = mouse_position();
        let position_in_pixels = Vec2::new(position_in_pixels_tuple.0, position_in_pixels_tuple.1);
        let position_minus_1_to_1 =
            position_in_pixels / Vec2::new(screen_width(), screen_height()) * 2.0 - 1.0;
        theme
            .materials
            .floor
            .set_uniform(POSITION_X_NAME, position_minus_1_to_1.x);
        theme
            .materials
            .floor
            .set_uniform(POSITION_Y_NAME, position_minus_1_to_1.y);
        theme
            .materials
            .floor
            .set_uniform(TILES, self.size.into::<Vec2>());
        theme
            .materials
            .floor
            .set_uniform(COLOR_BLACK, theme.palette.tiles_black);
        theme
            .materials
            .floor
            .set_uniform(COLOR_WHITE, theme.palette.tiles_white);
        let radar = self.referee.radar_v2_offset();
        theme.materials.floor.set_uniform_array(RADAR, &radar);

        let corners = horizontal_quad(
            Coord::new_i(0, 0).to_vec3(0.0),
            self.size.column_f(),
            self.size.row_f(),
        );
        draw_mesh(&mesh_texture_quad(corners, WHITE, None, false, true));
        gl_use_default_material();
    }

    fn checks_meshes(&self, theme: &Theme) -> Vec<Mesh> {
        let mut meshes = Vec::new();
        for (_team, kind_index) in self.in_check() {
            meshes.extend(mesh_cursor_width(
                self.pieces[kind_index].initial_pos.into(),
                theme.palette.check,
                SELECTION_HEIGHT,
                0.2,
            ));
        }
        meshes
    }
    fn turn_light_meshes(&self, theme: &Theme) -> Vec<Mesh> {
        let z = vec3(0.0, 0.0, 1.0) * 10.0;
        let coord_00 = vec3(0.0 + self.size.column as f32 * 0.5, 4.0, 6.0);
        let slope_direction = if self.referee.turn.is_white() {
            1.0
        } else {
            -1.0
        };
        let xy = vec3(slope_direction, 1.0, 0.0) * 5.0;
        let corners = quad(coord_00, xy, z);
        let mut mesh = mesh_quad(corners, theme.palette.spotlight);
        mesh.vertices[0].color = TRANSPARENT.into();
        mesh.vertices[2].color = TRANSPARENT.into();
        vec![mesh]
    }
}
pub fn empty_tile(double_start: ICoord, piece_index: usize, pieces: &Vec<Piece>) -> bool {
    other_pieces_at(double_start, piece_index, pieces).len() == 0
}

pub fn cursor_color(team: Team, theme: &Theme) -> Color {
    if team.is_white() {
        theme.palette.cursor_white
    } else {
        theme.palette.cursor_black
    }
}

/// assumes meshes are just quads, with vertices in zig-zag order. (top left, top right, bottom left, bottom right).
fn depth(mesh: &Mesh) -> f32 {
    (mesh.vertices[0].position.z + mesh.vertices[2].position.z) * 0.5 * 0.001
        + (mesh.vertices[0].position.y + mesh.vertices[2].position.y) * 0.5 * 10.0
}
