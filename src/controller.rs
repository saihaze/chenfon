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
        RandomController {  }
    }
}

impl Controller for RandomController {
    fn decide(&self, side: Side,  board: &Board) -> Option<((i32, i32), (i32, i32))> {
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