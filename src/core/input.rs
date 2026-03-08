use crate::core::coord::Coord;
use crate::world::board::Board;
use crate::world::team::Team;
use gamepads::{Button, Gamepad};
use macroquad::math::vec2;

pub struct Gamepads {
    pub ids: Vec<(gamepads::GamepadId, Team)>,
    pub pads: gamepads::Gamepads,
}

impl Gamepads {
    pub fn new() -> Self {
        Self {
            ids: Vec::new(),
            pads: gamepads::Gamepads::new(),
        }
    }
    pub fn tick(&mut self) {
        self.pads.poll();
        let mut whites = 0;
        let mut blacks = 0;
        let mut new_ids = Vec::new();
        for pad in self.pads.all() {
            if let Some((_id, team)) = self.ids.iter().find(|(e, _)| *e == pad.id()) {
                if team.is_white() {
                    whites += 1;
                } else {
                    blacks += 1;
                }
            } else {
                new_ids.push(pad.id());
            }
        }
        for new_id in new_ids {
            let new_team = if whites <= blacks {
                whites += 1;
                Team::White
            } else {
                blacks += 1;
                Team::Black
            };
            self.ids.push((new_id, new_team));
        }
    }
    pub fn move_cursor_or_piece(&self, board: &mut Board) {
        if let Some((gamepad_id, team)) = self.ids.get(0) {
            maybe_move_team(self.pads.get(*gamepad_id).as_ref(), *team, board);
        }
        if let Some((gamepad_id, team)) = self.ids.get(1) {
            maybe_move_team(self.pads.get(*gamepad_id).as_ref(), *team, board);
        }
    }
}

fn maybe_move_team(gamepad: Option<&Gamepad>, team: Team, board: &mut Board) {
    if let Some(gamepad) = gamepad {
        move_team(gamepad, team, board);
    }
}
fn move_team(gamepad: &Gamepad, team: Team, board: &mut Board) {
    let max = 0.05;
    if gamepad.is_just_pressed(Button::ActionDown)
        || gamepad.is_just_pressed(Button::FrontLeftUpper)
        || gamepad.is_just_pressed(Button::FrontRightUpper)
    {
        board.toggle_select(team);
    }
    if board.is_selected(team) {
        let mut delta = vec2(gamepad.left_stick_x(), -gamepad.left_stick_y());
        delta += vec2(gamepad.right_stick_x(), gamepad.right_stick_y());
        if delta.length() < 0.1 {
            // protect against stick drift
            delta = vec2(0.0, 0.0);
        }
        if gamepad.is_currently_pressed(Button::DPadRight) {
            delta += vec2(0.1, 0.0);
        }
        if gamepad.is_currently_pressed(Button::DPadLeft) {
            delta += vec2(-0.1, 0.0);
        }
        if gamepad.is_currently_pressed(Button::DPadUp) {
            delta += vec2(0.0, -0.1);
        }
        if gamepad.is_currently_pressed(Button::DPadDown) {
            delta += vec2(0.0, 0.1);
        }
        if delta != vec2(0.0, 0.0) {
            delta = delta.normalize();
            delta *= max;
            board.move_cursor_rel(delta.into(), team);
        }
    } else {
        if gamepad.is_just_pressed(Button::DPadRight) {
            board.move_cursor_rel(Coord::new_i(1, 0), team);
        }
        if gamepad.is_just_pressed(Button::DPadLeft) {
            board.move_cursor_rel(Coord::new_i(-1, 0), team);
        }
        if gamepad.is_just_pressed(Button::DPadUp) {
            board.move_cursor_rel(Coord::new_i(0, -1), team);
        }
        if gamepad.is_just_pressed(Button::DPadDown) {
            board.move_cursor_rel(Coord::new_i(0, 1), team);
        }
    }
}
