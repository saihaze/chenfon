use crate::game::*;

/// AI 決定
#[derive(Debug, Clone, Copy)]
pub struct AIDecision {
    pub step: Option<((i32, i32), (i32, i32))>,
    pub score: f32,
}

/// 估價器
pub trait Evaluator {
    /// 估價
    fn evaluate(&self, board: &Board, side: Side) -> f32;
}

/// 短視估價器——將所有棋子分數加起來
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortSightedEvaluator {}

impl ShortSightedEvaluator {
    pub fn new() -> Self {
        Self {}
    }

    // 前中期估價單個棋子
    fn evaluate_single_piece_1(board: &Board, side: Side, pos: (i32, i32)) -> i32 {
        let piece = board.get_map()[pos.0 as usize][pos.1 as usize];
        match piece {
            Some(piece) => {
                let abs_value = match piece.1 {
                    Piece::兵 => 10,
                    Piece::仕 => 30,
                    Piece::相 => 30,
                    Piece::炮 => 55,
                    Piece::馬 => 50,
                    Piece::車 => 100,
                    Piece::帥 => 1000,
                };
                if side == piece.0 {
                    abs_value
                } else {
                    -abs_value
                }
            }
            None => 0,
        }
    }

    // 殘局估價單個棋子
    fn evaluate_single_piece_2(board: &Board, side: Side, pos: (i32, i32)) -> i32 {
        let piece = board.get_map()[pos.0 as usize][pos.1 as usize];
        match piece {
            Some(piece) => {
                let abs_value = match piece.1 {
                    Piece::兵 => 10,
                    Piece::仕 => 30,
                    Piece::相 => 30,
                    Piece::炮 => 50,
                    Piece::馬 => 55,
                    Piece::車 => 100,
                    Piece::帥 => 1000,
                };
                if side == piece.0 {
                    abs_value
                } else {
                    -abs_value
                }
            }
            None => 0,
        }
    }
}

impl Evaluator for ShortSightedEvaluator {
    fn evaluate(&self, board: &Board, side: Side) -> f32 {
        let mut score = 0;
        if board.piece_count_of_board() <= 16 {
            for x in 0..9 {
                for y in 0..10 {
                    score += Self::evaluate_single_piece_1(board, side, (x, y));
                }
            }
        } else {
            for x in 0..9 {
                for y in 0..10 {
                    score += Self::evaluate_single_piece_2(board, side, (x, y));
                }
            }
        }
        if side == Side::Red && board.get_map()[4][3] == Some((Side::Red, Piece::兵))
            || side == Side::Black && board.get_map()[4][3] == Some((Side::Black, Piece::兵))
        {
            score += 10;
        }
        if side == Side::Red && board.get_map()[4][3] == Some((Side::Black, Piece::兵))
            || side == Side::Black && board.get_map()[4][3] == Some((Side::Red, Piece::兵))
        {
            score -= 10;
        }
        score as f32 / 4000.0f32 + 0.5f32
    }
}

/// 最大-最小算法之最大
pub fn max_search<EvaluatorT>(
    depth: u32,
    current_node_count: &mut u32,
    max_node_count: u32,
    board: &mut Board,
    side: Side,
    evaluator: &EvaluatorT,
    mut alpha: f32,
    beta: f32,
) -> Option<AIDecision>
where
    EvaluatorT: Evaluator,
{
    if *current_node_count > max_node_count {
        return None;
    }
    *current_node_count += 1;
    if depth == 0 || board.finished() {
        Some(AIDecision {
            step: None,
            score: evaluator.evaluate(board, side),
        })
    } else {
        let mut score = std::f32::NEG_INFINITY;
        let mut step = ((0, 0), (0, 0));
        for x in 0..9 {
            for y in 0..10 {
                let from = (x, y);
                if !board.has_friend_at(side, from) {
                    continue;
                }
                for to in board.all_possible_moves(from) {
                    board.do_move_unchecked(from, to);
                    let v = min_search(
                        depth - 1,
                        current_node_count,
                        max_node_count,
                        board,
                        side,
                        evaluator,
                        alpha,
                        beta,
                    )?;
                    if v.score > score {
                        score = v.score;
                        step = (from, to);
                    }
                    if score > alpha {
                        alpha = score;
                    }
                    board.undo_move().unwrap();
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }
        if score > std::f32::NEG_INFINITY {
            Some(AIDecision {
                step: Some(step),
                score,
            })
        } else {
            Some(AIDecision {
                step: None,
                score: evaluator.evaluate(board, side),
            })
        }
    }
}

/// 最大-最小算法之最小
pub fn min_search<EvaluatorT>(
    depth: u32,
    current_node_count: &mut u32,
    max_node_count: u32,
    board: &mut Board,
    side: Side,
    evaluator: &EvaluatorT,
    alpha: f32,
    mut beta: f32,
) -> Option<AIDecision>
where
    EvaluatorT: Evaluator,
{
    if *current_node_count > max_node_count {
        return None;
    }
    *current_node_count += 1;
    if depth == 0 || board.finished() {
        Some(AIDecision {
            step: None,
            score: evaluator.evaluate(board, side),
        })
    } else {
        let mut score = std::f32::INFINITY;
        let mut step = ((0, 0), (0, 0));
        for x in 0..9 {
            for y in 0..10 {
                let from = (x, y);
                if !board.has_friend_at(side.other(), from) {
                    continue;
                }
                for to in board.all_possible_moves(from) {
                    board.do_move_unchecked(from, to);
                    let v = max_search(
                        depth - 1,
                        current_node_count,
                        max_node_count,
                        board,
                        side,
                        evaluator,
                        alpha,
                        beta,
                    )?;
                    if v.score < score {
                        score = v.score;
                        step = (from, to);
                    }
                    if score < beta {
                        beta = score;
                    }
                    board.undo_move().unwrap();
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }
        if score < std::f32::INFINITY {
            Some(AIDecision {
                step: Some(step),
                score,
            })
        } else {
            Some(AIDecision {
                step: None,
                score: evaluator.evaluate(board, side),
            })
        }
    }
}
