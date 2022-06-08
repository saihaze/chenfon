use crate::evaluator::*;
use crate::game::*;
use rand::prelude::*;

/// 控制器
pub trait Controller {
    fn decide(&self, side: Side, board: &Board) -> Option<((i32, i32), (i32, i32))>;
}

/// 隨機走子控制器
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RandomController {}

impl RandomController {
    /// 構建
    pub fn new() -> RandomController {
        RandomController {}
    }
}

impl Controller for RandomController {
    fn decide(&self, side: Side, board: &Board) -> Option<((i32, i32), (i32, i32))> {
        let mut froms: Vec<(i32, i32)> = Vec::new();
        for x in 0..9 {
            for y in 0..10 {
                let pos: (i32, i32) = (x, y);
                if !board.has_friend_at(side, pos) {
                    continue;
                }
                let possible_moves = board.all_possible_moves(pos);
                if !possible_moves.is_empty() {
                    froms.push(pos);
                }
            }
        }
        if froms.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            froms.shuffle(&mut rng);
            let from = froms.first().unwrap().clone();
            let mut tos = board.all_possible_moves(from);
            tos.shuffle(&mut rng);
            let to = tos.first().unwrap().clone();
            Some((from, to))
        }
    }
}

/// AI 控制器
#[derive(Debug)]
pub struct AIController<EvaluatorT>
where
    EvaluatorT: Evaluator,
{
    evaluator: EvaluatorT,
}

impl<EvaluatorT> AIController<EvaluatorT>
where
    EvaluatorT: Evaluator,
{
    /// 構建
    pub fn new(evaluator: EvaluatorT) -> AIController<EvaluatorT> {
        AIController { evaluator }
    }
}

impl<EvaluatorT> Controller for AIController<EvaluatorT>
where
    EvaluatorT: Evaluator
{
    fn decide(&self, side: Side, board: &Board) -> Option<((i32, i32), (i32, i32))> {
        let mut board = board.clone();
        let mut score = -100000;
        let mut ret = ((0, 0), (0, 0));
        for x in 0..9 {
            for y in 0..10 {
                let from = (x, y);
                if !board.has_friend_at(side, from) {
                    continue;
                }
                let steps = board.all_possible_moves(from);
                for to in steps {
                    board.do_move_unchecked(from, to);
                    let v = -self.evaluator.evaluate(side.other(), &board);
                    if v > score {
                        score = v;
                        ret = (from, to);
                    }
                    board.undo_move().unwrap();
                }
            }
        }
        if score == -100000 {
            None
        } else {
            Some(ret)
        }
    }
}
