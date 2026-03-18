use crate::core::coord::Coord;
use crate::world::board::Board;
use crate::world::bot::quantize;
use crate::world::team::Team;
use gamepads::Button;
use macroquad::math::{vec2, Vec2};

type JoystickWasAtRest = bool;

pub struct Gamepad {
    pub inner: gamepads::Gamepad,
    pub team: Team,
    pub joystick_rest: JoystickWasAtRest,
}

impl Gamepad {
    pub fn left_stick(&self) -> Vec2 {
        vec2(self.inner.left_stick_x(), -self.inner.left_stick_y())
    }
    pub fn right_stick(&self) -> Vec2 {
        vec2(self.inner.right_stick_x(), -self.inner.right_stick_y())
    }
}

pub struct Gamepads {
    pub cached: Vec<Gamepad>,
    pub pads: gamepads::Gamepads,
}

impl Gamepads {
    pub fn new() -> Self {
        Self {
            cached: Vec::new(),
            pads: gamepads::Gamepads::new(),
        }
    }
    pub fn tick(&mut self) {
        self.pads.poll();
        let mut whites = 0;
        let mut blacks = 0;
        let mut new_gamepads = Vec::new();
        println!("pad ids:");
        for pad in self.pads.all() {
            println!("pad id: {:?}", pad.id());

            if let Some(gamepad) = self
                .cached
                .iter_mut()
                .find(|gamepad| gamepad.inner.id() == pad.id())
            {
                gamepad.inner = pad;
                if gamepad.team.is_white() {
                    whites += 1;
                } else {
                    blacks += 1;
                }
            } else {
                new_gamepads.push(pad);
            }
        }
        for new_gamepad in new_gamepads {
            let new_team = if whites <= blacks {
                whites += 1;
                Team::White
            } else {
                blacks += 1;
                Team::Black
            };
            self.cached.push(Gamepad {
                inner: new_gamepad,
                team: new_team,
                joystick_rest: true,
            });
        }
    }
    pub fn move_cursor_or_piece(&mut self, board: &mut Board) {
        maybe_move_team(self.cached.get_mut(0), board);
        maybe_move_team(self.cached.get_mut(1), board);
    }
}

fn maybe_move_team(gamepad: Option<&mut Gamepad>, board: &mut Board) {
    if let Some(gamepad) = gamepad {
        move_team(gamepad, board);
    }
}
fn move_team(gamepad_outer: &mut Gamepad, board: &mut Board) {
    let mut delta = gamepad_outer.left_stick();
    delta += gamepad_outer.right_stick();
    let gamepad = &mut gamepad_outer.inner;
    let team = gamepad_outer.team;
    let max = 0.05;
    if gamepad.is_just_pressed(Button::ActionDown)
        || gamepad.is_just_pressed(Button::FrontLeftUpper)
        || gamepad.is_just_pressed(Button::FrontRightUpper)
    {
        board.toggle_select(team);
    }
    if board.is_selected(team) {
        gamepad_outer.joystick_rest = true;
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
        if let Some(movement) = maybe_move_joystick(&mut gamepad_outer.joystick_rest, delta) {
            // println!("movement: {:?}, delta: {}, team: {}", movement, delta, team);
            board.move_cursor_rel(movement, team);
        }
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

fn maybe_move_joystick(joystick_rest: &mut bool, delta: Vec2) -> Option<Coord> {
    let magnitude = delta.length();
    if magnitude > 0.6 {
        if *joystick_rest {
            *joystick_rest = false;
            return Some(quantize(delta.into()));
        }
    } else if magnitude < 0.3 {
        if !*joystick_rest {
            *joystick_rest = true;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_joystick() {
        let mut joystick_rest = true;
        let movement = maybe_move_joystick(&mut joystick_rest, vec2(0.0, 0.0));
        assert_eq!(movement, None);
        assert_eq!(joystick_rest, true);

        let movement = maybe_move_joystick(&mut joystick_rest, vec2(1.0, 0.0));
        assert_eq!(movement, Some(Coord::new_i(1, 0)));
        assert_eq!(joystick_rest, false);

        let movement = maybe_move_joystick(&mut joystick_rest, vec2(1.0, 0.0));
        assert_eq!(movement, None);
        assert_eq!(joystick_rest, false);

        let movement = maybe_move_joystick(&mut joystick_rest, vec2(0.0, 0.0));
        assert_eq!(movement, None);
        assert_eq!(joystick_rest, true);

        let movement = maybe_move_joystick(&mut joystick_rest, vec2(1.0, 0.0));
        assert_eq!(movement, Some(Coord::new_i(1, 0)));
        assert_eq!(joystick_rest, false);
    }
}
