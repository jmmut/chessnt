use crate::core::coord::Coord;
use crate::world::board::{other_pieces_at, Board, PieceIndex};
use crate::world::bot::{Plan, PlanSelect};
use crate::world::moves::{possible_moves, Move};
use crate::world::piece::Piece;
use crate::world::team::Team;
use std::time::Instant;

pub const PLANNING_DEPTH: i32 = 4;

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
pub fn choose_target_inner(team: Team, pieces: &Vec<Piece>, board_size: Coord) -> Option<Plan> {
    if let (Some((i, movement)), score) =
        choose_target_score(team, pieces, board_size, PLANNING_DEPTH)
    {
        Some(PlanSelect::new(i, movement))
    } else {
        choose_first_target_inner(team, pieces, board_size)
    }
}
pub fn choose_target_score(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: Coord,
    depth: i32,
) -> (Option<(PieceIndex, Coord)>, Score) {
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
    let initial_board_score: f32 = pieces.iter().map(|piece| piece_value(piece, team)).sum();
    if depth <= 0 {
        return (None, initial_board_score);
    }
    let mut working_board = pieces.clone();
    let mut best = None;
    for (i, piece) in working_board.iter().enumerate() {
        if piece.team == team {
            for movement in possible_moves(board_size, &working_board, i) {
                if let Some(other_i) = other_pieces_at(movement, i, &working_board).first() {
                    let other = &working_board[*other_i];
                    if other.team != team {
                        let mut potential_pieces = working_board.clone();
                        potential_pieces[i].set_pos_and_initial(movement);
                        potential_pieces[*other_i].alive = false;
                        potential_pieces[*other_i].set_pos_and_initial(Coord::new_i(0, -2));

                        let (_, future_score) = choose_target_score(
                            team.opposite(),
                            &potential_pieces,
                            board_size,
                            depth - 1,
                        );
                        let future_score = -future_score;
                        if let Some((best_i, best_movement, best_score)) = best {
                            if best_score < future_score {
                                best = Some((i, movement, future_score));
                            }
                        } else {
                            best = Some((i, movement, future_score));
                        }
                    }
                } else {
                    // TODO: modify score due to our movement's benefit
                    let mut potential_pieces = working_board.clone();
                    potential_pieces[i].set_pos_and_initial(movement);
                    let (_, future_score) = choose_target_score(
                        team.opposite(),
                        &potential_pieces,
                        board_size,
                        depth - 1,
                    );

                    let future_score = -future_score;
                    if let Some((best_i, best_movement, best_score)) = best {
                        if best_score < future_score {
                            best = Some((i, movement, future_score));
                        }
                    } else {
                        best = Some((i, movement, future_score));
                    }
                }
            }
        }
    }
    if let Some((best_i, best_move, best_score)) = best {
        (Some((best_i, best_move)), best_score)
    } else {
        (None, initial_board_score)
    }
}
pub fn choose_first_target(board: &Board, team: Team) -> Option<Plan> {
    choose_target_inner(team, board.pieces(), board.size())
}
pub fn choose_first_target_inner(
    team: Team,
    pieces: &Vec<Piece>,
    board_size: Coord,
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
        let plan = choose_target_inner(Team::White, &pieces, size);
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
        let plan = choose_target_inner(Team::Black, &pieces, size);
        let king = find_first(Team::Black, Move::King, &pieces).unwrap();
        assert_eq!(plan, Some(PlanSelect::new(king, Coord::new_i(1, 1))));
    }
}
