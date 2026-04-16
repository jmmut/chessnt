use crate::AnyResult;
use crate::core::coord::{Coord, ICoord};
use crate::world::board::tracking::EverMoved;
use crate::world::board::{Board, PieceIndex, PieceIndexSmall};
use crate::world::bot::{Plan, PlanSelect};
use crate::world::moves::{
    Move, Moveset, PieceIndexes, board_to_str_indent, checked_index_at, index_at,
    pieces_to_str, possible_moves, possible_moves_matrix_mut, print_pieces, set_index_at,
    to_piece_index_matrix_small,
};
use crate::world::piece::Piece;
use crate::world::team::Team;
use macroquad::logging::info;
use macroquad::prelude::get_time;

pub const PLANNING_DEPTH: i32 = 4;

static mut EVALUATIONS: i32 = 0;

#[allow(unused)]
pub mod debug_level {
    pub const NO: i32 = 0;
    pub const PLAN: i32 = 5;
    pub const CONCISE: i32 = 10;
    pub const TREE: i32 = 20;
    pub const VERBOSE: i32 = 30;
}

#[cfg(debug_assertions)]
pub const DEBUG_PLANNING_GLOBAL: i32 = debug_level::CONCISE;

#[cfg(not(debug_assertions))]
pub const DEBUG_PLANNING_GLOBAL: i32 = debug_level::PLAN;

pub type Score = f32;

pub struct DebugState {
    plan: Vec<(PieceIndex, ICoord)>,
}
impl DebugState {
    pub fn new() -> Self {
        Self { plan: Vec::new() }
    }
}
pub fn choose_target(board: &Board, team: Team) -> AnyResult<Option<Plan>> {
    let start = get_time();
    let in_check = board.is_in_check(team).is_some();
    let mut debug = DebugState::new();
    if board.referee.turn != team && !in_check {
        Ok(None)
    } else {
        let plan_score = choose_target_board_depth::<DEBUG_PLANNING_GLOBAL>(
            board,
            team,
            PLANNING_DEPTH,
            &mut debug,
        );
        let score_str = if let Ok((_, Some(score))) = &plan_score {
            format!("{:>8.1}", score)
        } else {
            "unknown".to_string()
        };
        info!(
            "planning took {:>10.3}ms, expected score {}",
            (get_time() - start) * 1000.0,
            score_str,
        );
        if let Ok((plan, _)) = plan_score {
            Ok(plan)
        } else {
            Ok(None)
        }
    }
}

pub fn choose_target_board_depth<const DEBUG_PLANNING: i32>(
    board: &Board,
    team: Team,
    depth: i32,
    debug: &mut DebugState,
) -> AnyResult<(Option<Plan>, Option<Score>)> {
    choose_target_inner_depth_plan::<DEBUG_PLANNING>(
        team,
        board.pieces(),
        board.size(),
        &board.ever_moved(),
        board.referee.turn,
        depth,
        debug,
    )
}
pub fn choose_target_inner_depth<const DEBUG_PLANNING: i32>(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    turn: Team,
    depth: i32,
) -> AnyResult<(Option<Plan>, Option<Score>)> {
    let mut debug = DebugState::new();
    choose_target_inner_depth_plan::<DEBUG_PLANNING>(
        team, &pieces, board_size, ever_moved, turn, depth, &mut debug,
    )
}

fn choose_target_inner_depth_plan<const DEBUG_PLANNING: i32>(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: ICoord,
    ever_moved: &EverMoved,
    turn: Team,
    depth: i32,
    debug: &mut DebugState,
) -> AnyResult<(Option<Plan>, Option<Score>)> {
    if DEBUG_PLANNING > debug_level::NO {
        unsafe {
            EVALUATIONS = 0;
        }
    }
    let mut indexes = to_piece_index_matrix_small(pieces, board_size);
    let mut ever_moved = ever_moved.clone();
    let mut all_moves: Vec<Vec<ICoord>> = vec![vec![]; depth as usize + 1];
    for moves in &mut all_moves {
        moves.reserve(50);
    }
    if let (Some((i, movement)), score) = choose_target_score_mut::<DEBUG_PLANNING>(
        team,
        &mut pieces.clone(),
        board_size,
        turn,
        depth,
        &None,
        &mut ever_moved,
        &mut indexes,
        &mut all_moves,
        debug,
    )? {
        if DEBUG_PLANNING > debug_level::NO {
            println!("evaluations: {}", unsafe { EVALUATIONS });
            println!("score: {:?}", score);
        }
        if DEBUG_PLANNING >= debug_level::PLAN {
            println!("plan steps: {:?}", debug.plan);
        }
        Ok((Some(PlanSelect::new(i, movement)), Some(score)))
    } else if team == turn {
        Ok((
            choose_first_target_inner(team, pieces, board_size, &ever_moved),
            None,
        ))
    } else {
        Ok((None, None))
    }
}

pub fn choose_target_score_mut<const DEBUG_PLANNING: i32>(
    team: Team,
    pieces: &mut Vec<Piece>,
    board_size: ICoord,
    turn: Team,
    depth: i32,
    overall_best: &Option<(PieceIndex, ICoord, Score)>,
    ever_moved: &mut EverMoved,
    indexes: &mut PieceIndexes,
    all_moves: &mut Vec<Vec<ICoord>>,
    debug: &mut DebugState,
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
    let mut moves = std::mem::take(&mut all_moves[depth as usize]);
    let mut best = None;
    let mut debug_best_plan = DebugState::new();
    for i in 0..pieces.len() {
        if pieces[i].team == team && pieces[i].alive {
            if DEBUG_PLANNING >= debug_level::CONCISE {
                println!(
                    "{}. where to move piece {} {:?} at {:?}?",
                    ".*".repeat(depth as usize - 1),
                    pieces[i].team,
                    pieces[i].moveset.single(),
                    pieces[i].initial_pos
                );
            }
            moves.clear();
            possible_moves_matrix_mut(
                i, &pieces, board_size, indexes, ever_moved, &mut moves,
            );
            for movement in &*moves {
                let mut debug_here = DebugState::new();
                let movement = *movement;
                let is_better = evaluate_movement::<DEBUG_PLANNING>(
                    movement,
                    i,
                    team,
                    board_size,
                    turn,
                    depth,
                    overall_best,
                    &mut best,
                    ever_moved,
                    pieces,
                    indexes,
                    all_moves,
                    &mut debug_here,
                )?;
                if is_better && DEBUG_PLANNING >= debug_level::PLAN {
                    debug_best_plan = debug_here;
                }
                if let (Some((overall_i, _, overall_score)), Some((i, movement, score))) =
                    (overall_best, &best)
                {
                    let mut overall_score = *overall_score;
                    if pieces[*overall_i].team != pieces[*i].team {
                        overall_score = -overall_score;
                    }
                    if *score > overall_score {
                        all_moves[depth as usize] = std::mem::take(&mut moves);
                        return Ok((Some((*i, *movement)), *score));
                    }
                }
            }
        }
    }
    all_moves[depth as usize] = std::mem::take(&mut moves);
    if let Some((best_i, best_move, best_score)) = best {
        if DEBUG_PLANNING >= debug_level::CONCISE {
            println!(
                "{} best move for {} is {:?} to {:?} with score {}",
                ".*".repeat(depth as usize),
                team,
                pieces[best_i].moveset.single(),
                best_move,
                best_score
            )
        }
        if DEBUG_PLANNING >= debug_level::PLAN {
            debug.plan.push((best_i, best_move));
            debug.plan.extend(debug_best_plan.plan);
        }
        Ok((Some((best_i, best_move)), best_score))
    } else {
        Ok((None, 0.0))
    }
}

fn evaluate_movement<const DEBUG_PLANNING: i32>(
    movement: ICoord,
    i: usize,
    team: Team,
    board_size: ICoord,
    turn: Team,
    depth: i32,
    overall_best: &Option<(PieceIndex, ICoord, Score)>,
    best: &mut Option<(PieceIndex, ICoord, Score)>,
    ever_moved: &mut EverMoved,
    pieces: &mut Vec<Piece>,
    indexes: &mut PieceIndexes,
    all_moves: &mut Vec<Vec<ICoord>>,
    debug: &mut DebugState,
) -> AnyResult<bool> {
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
    Ok(
        if let Some(other_i) = other_killable(i, pieces, movement, indexes, ever_moved) {
            let other_i = other_i as usize;
            if pieces[other_i].team != team && turn == team {
                // don't think about killing when it's not your turn
                let kill_value = piece_value(&pieces[other_i], team);

                let future_score = if depth >= 2 {
                    let old_killed_pos = kill_in_caches(other_i, pieces, indexes);
                    let old_pos = pieces[i].initial_pos;
                    move_in_caches(i, old_pos, movement, pieces, indexes);
                    ever_moved.register_movement(i, pieces, old_pos, movement, board_size);

                    let (_, future_score) = choose_target_score_mut::<DEBUG_PLANNING>(
                        team.opposite(),
                        pieces,
                        board_size,
                        turn.opposite(),
                        depth - 1,
                        best,
                        ever_moved,
                        indexes,
                        all_moves,
                        debug,
                    )?;
                    if piece_change != 0.0 {
                        pieces[i].moveset = Moveset::new(Move::Pawn);
                    }
                    move_in_caches(i, movement, old_pos, pieces, indexes);
                    unkill_in_caches(other_i, old_killed_pos, pieces, indexes);
                    ever_moved.undo_movement(i);
                    future_score
                } else {
                    if DEBUG_PLANNING > debug_level::NO {
                        unsafe {
                            EVALUATIONS += 1;
                        }
                    }
                    0.0
                };
                let future_score = -future_score - kill_value + piece_change;
                maybe_store_better_and_debug::<DEBUG_PLANNING>(
                    depth,
                    pieces,
                    i,
                    movement,
                    future_score,
                    overall_best,
                    best,
                )
            } else {
                false
            }
        } else {
            // TODO: modify score due to our movement's benefit
            let future_score = if depth >= 2 {
                let old_pos = pieces[i].initial_pos;
                let castle_rook_index_and_pos_and_new_pos = if pieces[i].moveset.single()
                    == Move::King
                    && (movement - old_pos).length_squared() == 2 * 2
                {
                    let to_rook = (movement - old_pos) / 2;
                    if let Some(rook) = checked_index_at(old_pos + to_rook * 3, indexes) {
                        Some((rook, pieces[rook as usize].initial_pos, old_pos + to_rook))
                    } else if let Some(rook) = checked_index_at(old_pos + to_rook * 4, indexes) {
                        Some((rook, pieces[rook as usize].initial_pos, old_pos + to_rook))
                    } else {
                        return Err(format!(
                        "castling appeared possible but couldn't find rook at {:?} nor {:?}. board:\n{}",
                        to_rook * 3, to_rook * 4, pieces_to_str(pieces)
                    ).into());
                    }
                } else {
                    None
                };
                move_in_caches(i, old_pos, movement, pieces, indexes);
                ever_moved.register_movement(i, pieces, old_pos, movement, board_size);

                if let Some((rook, old_rook_pos, new_rook_pos)) =
                    castle_rook_index_and_pos_and_new_pos.clone()
                {
                    let rook = rook as usize;
                    move_in_caches(rook, old_rook_pos, new_rook_pos, pieces, indexes);
                }

                let (_, future_score) = choose_target_score_mut::<DEBUG_PLANNING>(
                    team.opposite(),
                    pieces,
                    board_size,
                    team.opposite(), // team: not a bug. on the first level we want to evaluate movements out of our turn before the other team moves
                    depth - 1,
                    &best,
                    ever_moved,
                    indexes,
                    all_moves,
                    debug,
                )?;

                if piece_change != 0.0 {
                    pieces[i].moveset = Moveset::new(Move::Pawn);
                }

                move_in_caches(i, movement, old_pos, pieces, indexes);
                ever_moved.undo_movement(i);

                if let Some((rook, old_rook_pos, new_rook_pos)) =
                    castle_rook_index_and_pos_and_new_pos.clone()
                {
                    let rook = rook as usize;
                    move_in_caches(rook, new_rook_pos, old_rook_pos, pieces, indexes);
                }

                let future_score = -future_score + piece_change;
                future_score
            } else {
                if DEBUG_PLANNING > debug_level::NO {
                    unsafe {
                        EVALUATIONS += 1;
                    }
                }
                0.0
            };
            maybe_store_better_and_debug::<DEBUG_PLANNING>(
                depth,
                pieces,
                i,
                movement,
                future_score,
                overall_best,
                best,
            )
        },
    )
}

fn other_killable(
    i: usize,
    pieces: &Vec<Piece>,
    movement: ICoord,
    indexes: &PieceIndexes,
    ever_moved: &EverMoved,
) -> Option<PieceIndex> {
    if let Some(other_i) = index_at(movement, indexes) {
        Some(other_i as PieceIndex)
    } else if pieces[i].moveset.single() == Move::Pawn
        && let Some(pawn_jumped) = index_at(
            ICoord::new_i(pieces[i].initial_pos.column, movement.row),
            &indexes,
        )
        && ever_moved.en_passantable(pawn_jumped as PieceIndex)
    {
        Some(pawn_jumped as PieceIndex)
    } else {
        None
    }
}

fn move_in_caches(
    i: usize,
    from: ICoord,
    to: ICoord,
    pieces: &mut Vec<Piece>,
    indexes: &mut PieceIndexes,
) {
    pieces[i].set_pos_and_initial_i(to);
    set_index_at(from, None, indexes);
    set_index_at(to, Some(i as PieceIndexSmall), indexes);
}

fn kill_in_caches(
    i: usize,
    pieces: &mut Vec<Piece>,
    indexes: &mut PieceIndexes,
) -> ICoord {
    pieces[i].alive = false;
    let old_killed_pos = pieces[i].initial_pos;
    pieces[i].set_pos_and_initial(Coord::new_i(0, -2));
    set_index_at(old_killed_pos, None, indexes);
    old_killed_pos
}

fn unkill_in_caches(
    i: usize,
    pos_before_dying: ICoord,
    pieces: &mut Vec<Piece>,
    indexes: &mut PieceIndexes,
) {
    pieces[i].alive = true;
    pieces[i].set_pos_and_initial_i(pos_before_dying);
    set_index_at(pos_before_dying, Some(i as PieceIndexSmall), indexes);
}

fn maybe_store_better_and_debug<const DEBUG_PLANNING: i32>(
    depth: i32,
    pieces: &Vec<Piece>,
    i: usize,
    movement: ICoord,
    future_score: Score,
    overall_best: &Option<(PieceIndex, ICoord, Score)>,
    best: &mut Option<(PieceIndex, ICoord, Score)>,
) -> bool {
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
            is_better
        } else {
            maybe_store_better(best, future_score, i, movement)
        }
    } else {
        maybe_store_better(best, future_score, i, movement)
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
    use crate::world::board::tests::parse_board;
    use crate::world::moves::tests::parse_pieces;
    use std::time::Instant;

    #[test]
    fn test_choose_basic_kill() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_pieces("
            -- -- wb --
            bk -- -- wr
        ");
        let plan = choose_target_inner_depth::<DEBUG_PLANNING_GLOBAL>(
            Team::White,
            &pieces,
            size,
            &ever_moved,
            Team::White,
            2,
        )
        .unwrap()
        .0;
        let rook = find_first(Team::White, Move::Rook, &pieces).unwrap();
        let king = find_first(Team::Black, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(rook, pieces[king].initial_pos)));
    }
    #[test]
    fn test_choose_basic_save() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_pieces("
            bk -- -- wr
            br -- -- wp
        ");
        let plan = choose_target_inner_depth::<DEBUG_PLANNING_GLOBAL>(
            Team::Black,
            &pieces,
            size,
            &ever_moved,
            Team::Black,
            2,
        )
        .unwrap()
        .0;
        let king = find_first(Team::Black, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(king, ICoord::new_i(1, 1))));
    }
    #[test]
    fn test_planning_accounts_for_castle() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_pieces("
            -- -- -- wp wr
            -- -- -- wp --
            -- -- -- wp --
            -- -- -- wr wk
            -- -- -- -- --
            bp -- -- -- bk
        ");
        let plan = choose_target_inner_depth::<DEBUG_PLANNING_GLOBAL>(
            Team::White,
            &pieces,
            size,
            &ever_moved,
            Team::White,
            3,
        )
        .unwrap()
        .0;
        let king = find_first(Team::White, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(king, ICoord::new_i(4, 1))));
    }
    #[test]
    fn test_recursive_castle() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_pieces("
            br -- -- -- wr
            -- -- -- -- --
            -- -- -- -- --
            bk -- -- -- wk
            br -- -- -- wr
        ");
        let plan = choose_target_inner_depth::<DEBUG_PLANNING_GLOBAL>(
            Team::White,
            &pieces,
            size,
            &ever_moved,
            Team::White,
            2,
        )
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
        let (size, pieces, ever_moved) = parse_pieces("
            -- wp -- bk --
            -- wr -- -- --
        ");
        let plan = choose_target_inner_depth::<DEBUG_PLANNING_GLOBAL>(
            Team::White,
            &pieces,
            size,
            &ever_moved,
            Team::White,
            3,
        )
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
        let (size, pieces, ever_moved) = parse_pieces("
            -- wp wr -- -- --
            bb -- -- -- bk --
        ");
        let plan = choose_target_inner_depth::<DEBUG_PLANNING_GLOBAL>(
            Team::White,
            &pieces,
            size,
            &ever_moved,
            Team::White,
            3,
        )
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

    #[test]
    fn test_castle_tracked_in_planning() {
        #[rustfmt::skip]
        let (size, pieces, ever_moved) = parse_pieces("
            wr br
            -- --
            wp --
            wp --
            wp wk
            wp --
            -- --
            -- --
            -- bk
            -- --
            -- --
            wr --
        ");
        let (plan, score) = choose_target_inner_depth::<{ debug_level::PLAN }>(
            Team::White,
            &pieces,
            size,
            &ever_moved,
            Team::White,
            5,
        )
        .unwrap();
        let rook = find_first(Team::White, Move::Rook, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(rook, ICoord::new_i(1, 0))),);
        assert_ne!(
            score,
            Some(piece_type_value(Move::King) + piece_type_value(Move::Rook))
        );
    }

    #[test]
    fn test_en_passant_in_planning() {
        #[rustfmt::skip]
        let board = parse_board("
            -- -- bh -- -- --
            -- -- bp -- br --
            -- -- -- -- wp wr
            -- bk -- -- -- wb
        ");
        let mut debug = DebugState::new();
        let (_, score) =
            choose_target_board_depth::<{ debug_level::PLAN }>(&board, Team::White, 3, &mut debug)
                .unwrap();
        let white_pawn = find_first(Team::White, Move::Pawn, board.pieces()).unwrap();
        let black_pawn = find_first(Team::Black, Move::Pawn, board.pieces()).unwrap();
        assert_eq!(
            (&debug.plan[0..2], score),
            (
                [
                    (white_pawn, ICoord { column: 2, row: 2 }),
                    (black_pawn, ICoord { column: 3, row: 2 }),
                ]
                .as_slice(),
                Some(2.0)
            ),
            "full plan: {:?}",
            debug.plan
        )
    }

    #[test]
    #[ignore]
    fn benchmark() {
        #[rustfmt::skip]
        let board = parse_board("
            br bp -- -- -- -- -- wr
            bh -- bb -- -- -- -- --
            bb bp -- -- -- -- -- --
            bk -- -- -- -- -- wp wk
            bq bp -- bh -- -- wp --
            -- -- -- -- -- -- -- --
            -- -- -- bb -- -- -- --
            br bp -- -- -- -- -- wr
        ");
        let depth = 6;
        let mut debug = DebugState::new();
        let start = Instant::now();
        let plan = choose_target_board_depth::<{ debug_level::PLAN }>(
            &board,
            Team::Black,
            depth,
            &mut debug,
        )
        .unwrap();
        println!(
            "For depth {} took: {:.3}ms",
            depth,
            (Instant::now() - start).as_secs_f64() * 1000.0
        );
        assert_eq!(plan.1, Some(-1.0));
        assert_eq!(
            debug.plan,
            vec![
                (1, ICoord { column: 2, row: 0 }),
                (2, ICoord { column: 6, row: 0 }),
                (3, ICoord { column: 2, row: 2 }),
                (2, ICoord { column: 5, row: 0 }),
                (0, ICoord { column: 1, row: 0 }),
                (2, ICoord { column: 2, row: 0 })
            ]
        )
        // latest:
        // For depth 6 took: 4300 ms
    }
}
