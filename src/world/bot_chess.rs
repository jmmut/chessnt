use crate::AnyResult;
use crate::core::coord::{Coord, ICoord};
use crate::world::board::{Board, EverMoved, PieceIndex, PieceIndexSmall};
use crate::world::bot::{Plan, PlanSelect};
use crate::world::moves::{
    Move, Moveset, board_to_str, board_to_str_indent, checked_index_at, index_at, pieces_to_str,
    possible_moves, possible_moves_matrix_mut, print_pieces, set_index_at, set_occupied,
    to_occupied_matrix, to_piece_index_matrix_small,
};
use crate::world::piece::Piece;
use crate::world::team::Team;
use std::time::Instant;

pub const PLANNING_DEPTH: i32 = 4;

#[allow(unused)]
mod debug_level {
    pub const NO: i32 = 0;
    pub const CONCISE: i32 = 10;
    pub const TREE: i32 = 20;
    pub const VERBOSE: i32 = 30;
}

#[cfg(test)]
pub const DEBUG_PLANNING: i32 = debug_level::CONCISE;

#[cfg(not(test))]
pub const DEBUG_PLANNING: i32 = debug_level::NO;

pub type Score = f32;

pub fn choose_target(board: &Board, team: Team) -> AnyResult<Option<Plan>> {
    let start = Instant::now();
    let in_check = board.is_in_check(team).is_some();
    if board.referee.turn != team && !in_check {
        Ok(None)
    } else {
        let plan_score = choose_target_inner(
            team,
            board.pieces(),
            board.referee.turn,
            board.size(),
            board.ever_moved(),
        );
        let score_str = if let Ok((_, Some(score))) = &plan_score {
            format!("{:6.1}", score)
        } else {
            "unknown".to_string()
        };
        println!(
            "planning took {:5.3}ms, expected score {:6.1}",
            (Instant::now() - start).as_secs_f64() * 1000.0,
            score_str,
        );
        if let Ok((plan, _)) = plan_score {
            Ok(plan)
        } else {
            Ok(None)
        }
    }
}
pub fn choose_target_inner(
    team: Team,
    pieces: &Vec<Piece>,
    turn: Team,
    board_size: ICoord,
    ever_moved: &EverMoved,
) -> AnyResult<(Option<Plan>, Option<Score>)> {
    choose_target_inner_depth(team, pieces, board_size, ever_moved, turn, PLANNING_DEPTH)
}

pub fn choose_target_inner_depth(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    turn: Team,
    depth: i32,
) -> AnyResult<(Option<Plan>, Option<Score>)> {
    let mut occupied = to_occupied_matrix(pieces, board_size);
    let mut indexes = to_piece_index_matrix_small(pieces, board_size);
    if let (Some((i, movement)), score) = choose_target_score_mut(
        team,
        &mut pieces.clone(),
        board_size,
        ever_moved,
        turn,
        depth,
        &None,
        &mut occupied,
        &mut indexes,
    )? {
        Ok((Some(PlanSelect::new(i, movement)), Some(score)))
    } else if team == turn {
        Ok((
            choose_first_target_inner(team, pieces, board_size, ever_moved),
            None,
        ))
    } else {
        Ok((None, None))
    }
}
pub fn choose_target_score_mut(
    team: Team,
    pieces: &mut Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    turn: Team,
    depth: i32,
    overall_best: &Option<(PieceIndex, ICoord, Score)>,
    occupied: &mut Vec<Vec<Option<Team>>>,
    indexes: &mut Vec<Vec<Option<PieceIndexSmall>>>,
) -> AnyResult<(Option<(PieceIndex, ICoord)>, Score)> {
    if DEBUG_PLANNING >= debug_level::TREE {
        // print!("{}choosing move for board as {}:\n{}", ".".repeat(depth as usize), team, board_to_str(pieces));
        print!("\n{}", board_to_str_indent(pieces, board_size, depth - 1));
    }
    if depth <= 0 {
        if DEBUG_PLANNING >= debug_level::VERBOSE {
            println!(
                "{} returning (depth==0) with score {} for {} ************",
                ".*".repeat(depth as usize),
                0.0,
                team
            );
        }
        return Ok((None, 0.0));
    }
    let mut moves = Vec::new();
    let mut best = None;
    for i in 0..pieces.len() {
        if pieces[i].team == team && pieces[i].alive {
            if DEBUG_PLANNING >= debug_level::VERBOSE {
                println!(
                    "{}. where to move piece {} {:?} at {:?}?",
                    ".*".repeat(depth as usize - 1),
                    pieces[i].team,
                    pieces[i].moveset.single(),
                    pieces[i].initial_pos
                );
            }
            moves.clear();
            possible_moves_matrix_mut(i, &pieces, board_size, &occupied, ever_moved, &mut moves);
            for movement in &moves {
                let movement = *movement;
                evaluate_movement(
                    team,
                    pieces,
                    board_size,
                    ever_moved,
                    turn,
                    depth,
                    occupied,
                    indexes,
                    &mut best,
                    overall_best,
                    i,
                    movement,
                )?;
            }
        }
    }
    if let Some((best_i, best_move, best_score)) = best {
        Ok((Some((best_i, best_move)), best_score))
    } else {
        Ok((None, 0.0))
    }
}

fn evaluate_movement(
    team: Team,
    pieces: &mut Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    turn: Team,
    depth: i32,
    occupied: &mut Vec<Vec<Option<Team>>>,
    indexes: &mut Vec<Vec<Option<PieceIndexSmall>>>,
    best: &mut Option<(PieceIndex, ICoord, Score)>,
    overall_best: &Option<(PieceIndex, ICoord, Score)>,
    i: usize,
    movement: ICoord,
) -> AnyResult<()> {
    if DEBUG_PLANNING >= debug_level::VERBOSE {
        println!(
            "{} evaluating move to {:?}",
            ".*".repeat(depth as usize - 1),
            movement
        );
    }
    let piece_change = if pieces[i].moveset.single() == Move::Pawn {
        if team == Team::White && movement.column == 0
            || team == Team::Black && movement.column == board_size.column - 1
        {
            // TODO: allow user to choose promotion
            pieces[i].moveset = Moveset::new(Move::Queen);
            piece_type_value(Move::Queen) - piece_type_value(Move::Pawn)
        } else {
            0.0
        }
    } else {
        0.0
    };
    if let Some(other_i) = index_at(movement, &indexes) {
        let other_i = other_i as usize;
        if pieces[other_i].team != team {
            if turn != team {
                return Ok(()); // don't think about killing when it's not your turn
            }
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
                    ever_moved,
                    turn.opposite(),
                    depth - 1,
                    overall_best,
                    occupied,
                    indexes,
                )?;
                if piece_change != 0.0 {
                    pieces[i].moveset = Moveset::new(Move::Pawn);
                }

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
            let future_score = -future_score - kill_value + piece_change;
            maybe_store_better_and_debug(
                depth,
                pieces,
                i,
                movement,
                future_score,
                overall_best,
                best,
            );
        }
    } else {
        // TODO: modify score due to our movement's benefit
        let future_score = if depth >= 2 {
            let old_pos = pieces[i].initial_pos;
            let castle_rook_index_and_pos_and_new_pos = if pieces[i].moveset.single() == Move::King
                && (movement - old_pos).length_squared() == 2 * 2
            {
                let to_rook = (movement - old_pos) / 2;
                if let Some(rook) = checked_index_at(old_pos + to_rook * 3, indexes) {
                    Some((rook, pieces[rook as usize].initial_pos, old_pos + to_rook))
                } else if let Some(rook) = checked_index_at(old_pos + to_rook * 4, indexes) {
                    Some((rook, pieces[rook as usize].initial_pos, old_pos + to_rook))
                } else {
                    return Err(format!("castling appeared possible but couldn't find rook at {:?} nor {:?}. board:\n{}", to_rook * 3, to_rook * 4, pieces_to_str(pieces)).into());
                }
            } else {
                None
            };
            pieces[i].set_pos_and_initial_i(movement);
            set_occupied(old_pos, None, occupied);
            set_occupied(movement, Some(team), occupied);
            set_index_at(old_pos, None, indexes);
            set_index_at(movement, Some(i as PieceIndexSmall), indexes);

            if let Some((rook, old_rook_pos, new_rook_pos)) =
                castle_rook_index_and_pos_and_new_pos.as_ref()
            {
                pieces[*rook as usize].set_pos_and_initial_i(*new_rook_pos);
                set_occupied(*old_rook_pos, None, occupied);
                set_occupied(*new_rook_pos, Some(team), occupied);
                set_index_at(*old_rook_pos, None, indexes);
                set_index_at(*old_rook_pos, Some(*rook), indexes);
            }

            let (_, future_score) = choose_target_score_mut(
                team.opposite(),
                pieces,
                board_size,
                ever_moved,
                team.opposite(), // team: not a bug. on the first level we want to evaluate movements out of our turn before the other team moves
                depth - 1,
                overall_best,
                occupied,
                indexes,
            )?;

            if piece_change != 0.0 {
                pieces[i].moveset = Moveset::new(Move::Pawn);
            }
            pieces[i].set_pos_and_initial_i(old_pos);
            set_occupied(old_pos, Some(team), occupied);
            set_occupied(movement, None, occupied);
            set_index_at(old_pos, Some(i as PieceIndexSmall), indexes);
            set_index_at(movement, None, indexes);

            if let Some((rook, old_rook_pos, new_rook_pos)) =
                castle_rook_index_and_pos_and_new_pos.as_ref()
            {
                pieces[*rook as usize].set_pos_and_initial_i(*old_rook_pos);
                set_occupied(*old_rook_pos, Some(team), occupied); // TODO: extract?
                set_occupied(*new_rook_pos, None, occupied);
                set_index_at(*old_rook_pos, Some(*rook), indexes);
                set_index_at(*old_rook_pos, None, indexes);
            }

            let future_score = -future_score + piece_change;
            future_score
        } else {
            0.0
        };
        maybe_store_better_and_debug(depth, pieces, i, movement, future_score, overall_best, best);
    }
    Ok(())
}

fn maybe_store_better_and_debug(
    depth: i32,
    pieces: &Vec<Piece>,
    i: usize,
    movement: ICoord,
    future_score: Score,
    overall_best: &Option<(PieceIndex, ICoord, Score)>,
    best: &mut Option<(PieceIndex, ICoord, Score)>,
) {
    if DEBUG_PLANNING >= debug_level::CONCISE {
        if let Some((best_i_until_now, best_move_until_now, score_until_now)) = best.clone() {
            let is_better = maybe_store_better(best, future_score, i, movement);

            if let Some((overall_best_i, overall_best_move, overall_best_score)) = overall_best
                && future_score.max(score_until_now) > *overall_best_score
            {
                println!(
                    "{} chose moving {} {:?} to {:?} with better score overall {} than {} ({} {:?} to {:?})",
                    ".*".repeat(depth as usize),
                    pieces[i].team,
                    pieces[i].moveset.single(),
                    movement,
                    future_score,
                    overall_best_score,
                    pieces[*overall_best_i].team,
                    pieces[*overall_best_i].moveset.single(),
                    overall_best_move,
                );
            } else if is_better && DEBUG_PLANNING >= debug_level::TREE {
                println!(
                    "{} chose moving {} {:?} to {:?} with better score {} than {} ({} {:?} to {:?})",
                    ".*".repeat(depth as usize),
                    pieces[i].team,
                    pieces[i].moveset.single(),
                    movement,
                    future_score,
                    score_until_now,
                    pieces[best_i_until_now].team,
                    pieces[best_i_until_now].moveset.single(),
                    best_move_until_now,
                );
            } else if DEBUG_PLANNING >= debug_level::VERBOSE {
                println!(
                    "{} discarded moving {} {:?} to {:?} with score {} (best is {} for {} {:?} to {:?}",
                    ".*".repeat(depth as usize),
                    pieces[i].team,
                    pieces[i].moveset.single(),
                    movement,
                    future_score,
                    score_until_now,
                    pieces[best_i_until_now].team,
                    pieces[best_i_until_now].moveset.single(),
                    best_move_until_now,
                )
            }
        } else {
            maybe_store_better(best, future_score, i, movement);
        }
    } else {
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
) -> bool {
    let is_better = if let Some((_, _, best_score)) = best {
        *best_score < future_score
    } else {
        true
    };
    if is_better {
        // print_decision_kill(pieces, i, movement, other_i, future_score, depth);
        *best = Some((i, movement, future_score));
    }
    is_better
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
    print_pieces(pieces);
    println!(
        "{}moving {} {:?} to {:?} (kill {} {:?}) has better score {}",
        ".".repeat(depth as usize),
        piece.team,
        piece.moveset.single(),
        movement,
        pieces[*other_i].team,
        pieces[*other_i].moveset.single(),
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
    print_pieces(pieces);
    println!(
        "{}moving {} {:?} to {:?} has better score {}",
        ".".repeat(depth as usize),
        piece.team,
        piece.moveset.single(),
        movement,
        future_score
    );
}

pub fn choose_first_target_inner(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
) -> Option<Plan> {
    for (i, piece) in pieces.iter().enumerate() {
        if piece.team == team {
            let moves = possible_moves(i, pieces, board_size, ever_moved);
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
        let movement = piece.moveset.single();
        let my_team = (piece.team == team) as i32 as f32 * 2.0 - 1.0;
        let magnitude = piece_type_value(movement);
        magnitude * my_team
    } else {
        0.0
    }
}

fn piece_type_value(movement: Move) -> f32 {
    match movement {
        Move::Pawn => 1.0,
        Move::Knight => 3.0,
        Move::Bishop => 4.0,
        Move::Rook => 5.0,
        Move::Queen => 8.0,
        Move::King => 10000.0,
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
        let (size, pieces, ever_moved) = parse_board("
            -- -- wb --
            bk -- -- wr
        ");
        let plan =
            choose_target_inner_depth(Team::White, &pieces, size, &ever_moved, Team::White, 2)
                .unwrap()
                .0;
        let rook = find_first(Team::White, Move::Rook, &pieces).unwrap();
        let king = find_first(Team::Black, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(rook, pieces[king].initial_pos)));
    }
    #[test]
    fn test_choose_basic_save() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_board("
            bk -- -- wr
            br -- -- wp
        ");
        let plan =
            choose_target_inner_depth(Team::Black, &pieces, size, &ever_moved, Team::Black, 2)
                .unwrap()
                .0;
        let king = find_first(Team::Black, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(king, ICoord::new_i(1, 1))));
    }
    #[test]
    fn test_planning_accounts_for_castle() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_board("
            -- -- -- wp wr
            -- -- -- wp --
            -- -- -- wp --
            -- -- -- wr wk
            -- -- -- -- --
            bp -- -- -- bk
        ");
        let plan =
            choose_target_inner_depth(Team::White, &pieces, size, &ever_moved, Team::White, 3)
                .unwrap()
                .0;
        let king = find_first(Team::White, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(king, ICoord::new_i(4, 1))));
    }
    #[test]
    fn test_recursive_castle() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_board("
            br -- -- -- wr
            -- -- -- -- --
            -- -- -- -- --
            bk -- -- -- wk
            br -- -- -- wr
        ");
        let plan =
            choose_target_inner_depth(Team::White, &pieces, size, &ever_moved, Team::White, 2)
                .unwrap()
                .0;
        let top_white_rook = find_first(Team::White, Move::Rook, &pieces).unwrap();
        assert_eq!(
            plan,
            Some(PlanSelect::new(top_white_rook, ICoord::new_i(0, 0)))
        );
    }

    #[test]
    fn test_pawn_promotion() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_board("
            -- wp -- bk --
            -- wr -- -- --
        ");
        let plan =
            choose_target_inner_depth(Team::White, &pieces, size, &ever_moved, Team::White, 3)
                .unwrap();
        let pawn = find_first(Team::White, Move::Pawn, &pieces).unwrap();
        assert_eq!(
            plan,
            (
                Some(PlanSelect::new(pawn, ICoord::new_i(0, 0))),
                Some(
                    piece_type_value(Move::King) + piece_type_value(Move::Queen)
                        - piece_type_value(Move::Pawn)
                )
            ),
        );
    }
    #[test]
    fn test_pawn_killing_promotion() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_board("
            -- wp wr -- -- --
            bb -- -- -- bk --
        ");
        let plan =
            choose_target_inner_depth(Team::White, &pieces, size, &ever_moved, Team::White, 3)
                .unwrap();
        let pawn = find_first(Team::White, Move::Pawn, &pieces).unwrap();
        assert_eq!(
            plan,
            (
                Some(PlanSelect::new(pawn, ICoord::new_i(0, 1))),
                Some(
                    piece_type_value(Move::King)
                        + piece_type_value(Move::Bishop)
                        + piece_type_value(Move::Queen)
                        - piece_type_value(Move::Pawn)
                )
            ),
        );
    }
}
