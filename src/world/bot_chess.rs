use crate::core::coord::{Coord, ICoord};
use crate::world::board::{other_pieces_at, Board, PieceIndex};
use crate::world::bot::{Plan, PlanSelect};
use crate::world::moves::{board_to_str, is_better, possible_moves, print_board, Move};
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
    if let (Some((i, movement)), _score) =
        choose_target_score(team, &mut pieces.clone(), board_size, depth)
    {
        Some(PlanSelect::new(i, movement))
    } else {
        choose_first_target_inner(team, pieces, board_size)
    }
}
pub fn choose_target_score(
    team: Team,
    pieces: &mut Vec<Piece>,
    board_size: ICoord,
    depth: i32,
) -> (Option<(PieceIndex, ICoord)>, Score) {
    // make pieces a Vec<&Piece> to be able to replace pieces easily?
    // evaluate board
    // for each piece
    //   for each movement
    //     update board with piece moved  <--- ??
    //     if evaluate board is better
    //       store best
    // return best

    // make pieces a Vec<&Piece> to be able to replace pieces easily?
    // evaluate pieces values
    // for each piece
    //   for each movement
    //     update piece value (-old +new)
    //     if evaluate board is better
    //       store best
    // return best
    let initial_board_score: f32 = evaluate_pieces(team, pieces);
    if DEBUG_PLANNING {
        // print!("{}choosing move for board as {}:\n{}", ".".repeat(depth as usize), team, board_to_str(pieces));
        print!("{}", board_to_str(pieces));
    }
    if depth <= 0 {
        if DEBUG_PLANNING {
            println!(
                "{} returning (depth==0) with score {} for {} ************",
                ".*".repeat(depth as usize),
                initial_board_score,
                team
            );
        }
        return (None, initial_board_score);
    }
    let mut best = None;
    for i in 0..pieces.len() {
        if pieces[i].team == team {
            if DEBUG_PLANNING {
                println!(
                    "{}. where to move piece {} {:?} at {:?}?",
                    ".*".repeat(depth as usize - 1),
                    pieces[i].team,
                    pieces[i].moveset.first().unwrap(),
                    pieces[i].initial_pos
                );
            }
            for movement in possible_moves(board_size, &pieces, i) {
                if DEBUG_PLANNING {
                    println!(
                        "{} evaluating move to {:?}",
                        ".*".repeat(depth as usize - 1),
                        movement
                    );
                }
                if let Some(other_i) = other_pieces_at(movement, i, &pieces).first() {
                    let other = &pieces[*other_i];
                    if other.team != team {
                        let old_pos = pieces[i].initial_pos;
                        pieces[i].set_pos_and_initial_i(movement);
                        pieces[*other_i].alive = false;
                        let old_killed_pos = pieces[*other_i].initial_pos;
                        pieces[*other_i].set_pos_and_initial(Coord::new_i(0, -2));

                        let (_, future_score) =
                            choose_target_score(team.opposite(), pieces, board_size, depth - 1);

                        pieces[i].set_pos_and_initial_i(old_pos);
                        pieces[*other_i].set_pos_and_initial_i(old_killed_pos);
                        pieces[*other_i].alive = true;

                        let future_score = -future_score;
                        // if let Some((best_i, best_movement, best_score)) = best {
                        //     if best_score < future_score {
                        //         print_decision_kill(pieces, i, movement, other_i, future_score, depth);
                        //         best = Some((i, movement, future_score));
                        //     }
                        // } else {
                        //     print_decision_kill(pieces, i, movement, other_i, future_score, depth);
                        //     best = Some((i, movement, future_score));
                        // }

                        maybe_store_better(&mut best, future_score, i, movement);
                    }
                } else {
                    // TODO: modify score due to our movement's benefit
                    let old_pos = pieces[i].initial_pos;
                    pieces[i].set_pos_and_initial_i(movement);
                    let (_, future_score) =
                        choose_target_score(team.opposite(), pieces, board_size, depth - 1);
                    pieces[i].set_pos_and_initial_i(old_pos);

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
                    maybe_store_better(&mut best, future_score, i, movement);
                }
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
        (None, initial_board_score)
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
    if is_better(best, future_score, |(_, _, best_score), future_score| {
        best_score < future_score
    }) {
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
