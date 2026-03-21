use crate::core::coord::{Coord, ICoord};
use crate::world::board::{Board, PieceIndex, PieceIndexSmall, other_pieces_at};
use crate::world::bot::{Plan, PlanSelect};
use crate::world::moves::{
    Move, board_to_str, index_at, is_better, possible_moves, possible_moves_matrix,
    possible_moves_matrix_mut, print_board, set_index_at, set_occupied, to_occupied_matrix,
    to_piece_index_matrix, to_piece_index_matrix_small,
};
use crate::world::piece::Piece;
use crate::world::team::Team;
use std::time::Instant;

pub const PLANNING_DEPTH: i32 = 4;

#[cfg(test)]
pub const DEBUG_PLANNING: bool = true;

#[cfg(not(test))]
pub const DEBUG_PLANNING: bool = false;

pub type Score = f32;

pub fn choose_target(board: &Board, team: Team) -> Option<Plan> {
    let start = Instant::now();
    let plan = choose_target_inner(team, board.pieces(), board.size());
    println!(
        "planning took {:5.3}ms",
        (Instant::now() - start).as_secs_f64() * 1000.0
    );
    plan
}
pub fn choose_target_inner(team: Team, pieces: &Vec<Piece>, board_size: ICoord) -> Option<Plan> {
    choose_target_inner_depth(team, pieces, board_size, PLANNING_DEPTH)
}

pub fn choose_target_inner_depth(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    depth: i32,
) -> Option<Plan> {
    let mut occupied = to_occupied_matrix(pieces, board_size);
    let mut indexes = to_piece_index_matrix_small(pieces, board_size);
    if let (Some((i, movement)), _score) = choose_target_score_mut(
        team,
        &mut pieces.clone(),
        board_size,
        depth,
        &mut occupied,
        &mut indexes,
    ) {
        Some(PlanSelect::new(i, movement))
    } else {
        choose_first_target_inner(team, pieces, board_size)
    }
}
pub fn choose_target_score_mut(
    team: Team,
    pieces: &mut Vec<Piece>,
    board_size: ICoord,
    depth: i32,
    occupied: &mut Vec<Vec<Option<Team>>>,
    indexes: &mut Vec<Vec<Option<PieceIndexSmall>>>,
) -> (Option<(PieceIndex, ICoord)>, Score) {
    if DEBUG_PLANNING {
        // print!("{}choosing move for board as {}:\n{}", ".".repeat(depth as usize), team, board_to_str(pieces));
        print!("{}", board_to_str(pieces));
    }
    if depth <= 0 {
        if DEBUG_PLANNING {
            println!(
                "{} returning (depth==0) with score {} for {} ************",
                ".*".repeat(depth as usize),
                0.0,
                team
            );
        }
        return (None, 0.0);
    }
    let mut moves = Vec::new();
    let mut best = None;
    for i in 0..pieces.len() {
        if pieces[i].team == team && pieces[i].alive {
            if DEBUG_PLANNING {
                println!(
                    "{}. where to move piece {} {:?} at {:?}?",
                    ".*".repeat(depth as usize - 1),
                    pieces[i].team,
                    pieces[i].moveset.first().unwrap(),
                    pieces[i].initial_pos
                );
            }
            moves.clear();
            possible_moves_matrix_mut(board_size, &pieces, i, &occupied, &mut moves);
            for movement in &moves {
                let movement = *movement;
                evaluate_movement(
                    team, pieces, board_size, depth, occupied, indexes, &mut best, i, movement,
                );
            }
        }
    }
    if let Some((best_i, best_move, best_score)) = best {
        if DEBUG_PLANNING {
            println!(
                "{} chose moving {} {:?} to {:?} with score {}",
                ".*".repeat(depth as usize),
                pieces[best_i].team,
                pieces[best_i].moveset.first().unwrap(),
                best_move,
                best_score
            );
        }
        (Some((best_i, best_move)), best_score)
    } else {
        (None, 0.0)
    }
}

fn evaluate_movement(
    team: Team,
    pieces: &mut Vec<Piece>,
    board_size: ICoord,
    depth: i32,
    occupied: &mut Vec<Vec<Option<Team>>>,
    indexes: &mut Vec<Vec<Option<PieceIndexSmall>>>,
    best: &mut Option<(PieceIndex, ICoord, Score)>,
    i: usize,
    movement: ICoord,
) {
    if DEBUG_PLANNING {
        println!(
            "{} evaluating move to {:?}",
            ".*".repeat(depth as usize - 1),
            movement
        );
    }
    if let Some(other_i) = index_at(movement, &indexes) {
        let other_i = other_i as usize;
        if pieces[other_i].team != team {
            let kill_value = piece_value(&pieces[other_i], team);

            let future_score = if depth >= 2 {
                let old_pos = pieces[i].initial_pos;
                pieces[i].set_pos_and_initial_i(movement);
                pieces[other_i].alive = false;
                let old_killed_pos = pieces[other_i].initial_pos;
                pieces[other_i].set_pos_and_initial(Coord::new_i(0, -2));
                set_occupied(old_killed_pos, None, occupied);
                set_occupied(old_pos, None, occupied);
                set_occupied(movement, Some(team), occupied);
                set_index_at(old_killed_pos, None, indexes);
                set_index_at(old_pos, None, indexes);
                set_index_at(movement, Some(i as PieceIndexSmall), indexes);

                let (_, future_score) = choose_target_score_mut(
                    team.opposite(),
                    pieces,
                    board_size,
                    depth - 1,
                    occupied,
                    indexes,
                );

                pieces[i].set_pos_and_initial_i(old_pos);
                pieces[other_i].set_pos_and_initial_i(old_killed_pos);
                pieces[other_i].alive = true;
                set_occupied(old_pos, Some(team), occupied);
                set_occupied(movement, None, occupied);
                set_occupied(old_killed_pos, Some(pieces[other_i].team), occupied);
                set_index_at(old_pos, Some(i as PieceIndexSmall), indexes);
                set_index_at(movement, None, indexes);
                set_index_at(old_killed_pos, Some(other_i as PieceIndexSmall), indexes);

                future_score
            } else {
                0.0
            };
            let future_score = -future_score - kill_value;
            // if let Some((best_i, best_movement, best_score)) = best {
            //     if best_score < future_score {
            //         print_decision_kill(pieces, i, movement, other_i, future_score, depth);
            //         best = Some((i, movement, future_score));
            //     }
            // } else {
            //     print_decision_kill(pieces, i, movement, other_i, future_score, depth);
            //     best = Some((i, movement, future_score));
            // }

            maybe_store_better(best, future_score, i, movement);
        }
    } else {
        // TODO: modify score due to our movement's benefit
        let future_score = if depth >= 2 {
            let old_pos = pieces[i].initial_pos;
            pieces[i].set_pos_and_initial_i(movement);
            set_occupied(old_pos, None, occupied);
            set_occupied(movement, Some(team), occupied);
            set_index_at(old_pos, None, indexes);
            set_index_at(movement, Some(i as PieceIndexSmall), indexes);

            let (_, future_score) = choose_target_score_mut(
                team.opposite(),
                pieces,
                board_size,
                depth - 1,
                occupied,
                indexes,
            );

            pieces[i].set_pos_and_initial_i(old_pos);
            set_occupied(old_pos, Some(team), occupied);
            set_occupied(movement, None, occupied);
            set_index_at(old_pos, Some(i as PieceIndexSmall), indexes);
            set_index_at(movement, None, indexes);

            future_score
        } else {
            0.0
        };
        let future_score = -future_score;
        // if let Some((best_i, best_movement, best_score)) = best {
        //     if best_score < future_score {
        //         // print_decision(pieces, depth, piece, movement, future_score);
        //         best = Some((i, movement, future_score));
        //     }
        // } else {
        //     // print_decision(pieces, depth, piece, movement, future_score);
        //     best = Some((i, movement, future_score));
        // }
        maybe_store_better(best, future_score, i, movement);
    }
}

pub fn evaluate_pieces(team: Team, pieces: &Vec<Piece>) -> f32 {
    pieces.iter().map(|piece| piece_value(piece, team)).sum()
}

fn maybe_store_better(
    best: &mut Option<(PieceIndex, ICoord, Score)>,
    future_score: Score,
    i: PieceIndex,
    movement: ICoord,
) {
    let is_better = if let Some((_, _, best_score)) = best {
        *best_score < future_score
    } else {
        true
    };
    if is_better {
        // print_decision_kill(pieces, i, movement, other_i, future_score, depth);
        *best = Some((i, movement, future_score));
    }
}

#[allow(unused)]
fn print_decision_kill(
    pieces: &mut Vec<Piece>,
    i: PieceIndex,
    movement: Coord,
    other_i: &PieceIndex,
    future_score: Score,
    depth: i32,
) {
    let piece = &pieces[i];
    print_board(pieces);
    println!(
        "{}moving {} {:?} to {:?} (kill {} {:?}) has better score {}",
        ".".repeat(depth as usize),
        piece.team,
        piece.moveset.first().unwrap(),
        movement,
        pieces[*other_i].team,
        pieces[*other_i].moveset.first().unwrap(),
        future_score
    );
}

#[allow(unused)]
fn print_decision(
    pieces: &mut Vec<Piece>,
    depth: i32,
    piece: &Piece,
    movement: Coord,
    future_score: Score,
) {
    print_board(pieces);
    println!(
        "{}moving {} {:?} to {:?} has better score {}",
        ".".repeat(depth as usize),
        piece.team,
        piece.moveset.first().unwrap(),
        movement,
        future_score
    );
}

pub fn choose_first_target(board: &Board, team: Team) -> Option<Plan> {
    choose_target_inner(team, board.pieces(), board.size())
}
pub fn choose_first_target_inner(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
) -> Option<Plan> {
    for (i, piece) in pieces.iter().enumerate() {
        if piece.team == team {
            let moves = possible_moves(board_size, pieces, i);
            if let Some(movement) = moves.first() {
                return Some(PlanSelect::new(i, *movement));
            }
        }
    }
    None
}

/// if the team matches returns a positive number. opposite team return negative
pub fn piece_value(piece: &Piece, team: Team) -> f32 {
    if piece.alive {
        let movement = piece.moveset.first().unwrap();
        let my_team = (piece.team == team) as i32 as f32 * 2.0 - 1.0;
        let magnitude = match movement {
            Move::Pawn => 1.0,
            Move::Knight => 3.0,
            Move::Bishop => 4.0,
            Move::Rook => 5.0,
            Move::Queen => 8.0,
            Move::King => 10000.0,
        };
        magnitude * my_team
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::board::find_first;
    use crate::world::moves::tests::parse_board;

    #[test]
    fn test_choose_basic_kill() {
        #[rustfmt::skip]
        let (size, pieces) = parse_board("
            -- -- wb --
            bk -- -- wr
        ");
        let plan = choose_target_inner_depth(Team::White, &pieces, size, 2);
        let rook = find_first(Team::White, Move::Rook, &pieces).unwrap();
        let king = find_first(Team::Black, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(rook, pieces[king].initial_pos)));
    }
    #[test]
    fn test_choose_basic_save() {
        #[rustfmt::skip]
        let (size, pieces) = parse_board("
            bk -- -- wr
            br -- -- wp
        ");
        let plan = choose_target_inner_depth(Team::Black, &pieces, size, 2);
        let king = find_first(Team::Black, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(king, ICoord::new_i(1, 1))));
    }
}
