use std::io::stdin;

use crate::ai::*;
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

pub struct AIController<EvaluatorT>
where
    EvaluatorT: Evaluator,
{
    evaluator: EvaluatorT,
    max_node_count: u32,
}

impl<EvaluatorT> AIController<EvaluatorT>
where
    EvaluatorT: Evaluator,
{
    pub fn new(evaluator: EvaluatorT, max_node_count: u32) -> Self {
        Self {
            evaluator,
            max_node_count,
        }
    }
}

impl<EvaluatorT> Controller for AIController<EvaluatorT>
where
    EvaluatorT: Evaluator,
{
    fn decide(&self, side: Side, board: &Board) -> Option<((i32, i32), (i32, i32))> {
        let mut mboard = board.clone();
        let mut current_node_count = 0;
        let mut step = max_search(
            1,
            &mut current_node_count,
            self.max_node_count,
            &mut mboard,
            side,
            &self.evaluator,
            std::f32::NEG_INFINITY,
            std::f32::INFINITY,
        );
        match step {
            Some(_) => {
                for depth in 0..100 {
                    mboard = board.clone();
                    current_node_count = 0;
                    let nstep = max_search(
                        depth,
                        &mut current_node_count,
                        self.max_node_count,
                        &mut mboard,
                        side,
                        &self.evaluator,
                        std::f32::NEG_INFINITY,
                        std::f32::INFINITY,
                    );
                    match nstep {
                        Some(_) => {
                            step = nstep;
                            continue;
                        }
                        None => {
                            break;
                        }
                    }
                }
                Some(step.unwrap().step.unwrap().clone())
            }
            None => None,
        }
    }
}
