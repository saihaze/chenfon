use crate::game::*;

/// 估價器
pub trait Evaluator {
    /// 估價
    fn evaluate(&self, side: Side, board: &Board) -> i32;
}

/// 累加所有棋子分數
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortSightedEvaluator {}

impl ShortSightedEvaluator {
    pub fn new() -> ShortSightedEvaluator {
        ShortSightedEvaluator {}
    }
}

impl Evaluator for ShortSightedEvaluator {
    /// 估價
    fn evaluate(&self, side: Side, board: &Board) -> i32 {
        let mut ret = 0;
        let map = board.get_map();
        for col in map {
            for piece in col {
                ret += piece_relative_score(side, piece.clone());
            }
        }
        ret
    }
}

/// 搜索強化已有估價器
#[derive(Debug)]
pub struct EnhancedEvaluator<Original>
where
    Original: Evaluator,
{
    depth: u32,
    original: Original,
}

impl<Original> EnhancedEvaluator<Original>
where
    Original: Evaluator,
{
    /// 構建
    pub fn new(depth: u32, original: Original) -> EnhancedEvaluator<Original> {
        EnhancedEvaluator { depth, original }
    }

    /// 執行最大最小值搜索
    fn search(&self, depth: u32, side: Side, board: &mut Board) -> i32 {
        if depth == 0 || board.finished() {
            return self.original.evaluate(side, board);
        }
        let mut ret = -2000;
        for x in 0..9 {
            for y in 0..10 {
                let from = (x, y);
                if !board.has_friend_at(side, from) {
                    continue;
                }
                let steps = board.all_possible_moves(from);
                for to in steps {
                    board.do_move_unchecked(from, to);
                    let score = -self.search(depth - 1, side.other(), board);
                    if score > ret {
                        ret = score;
                    }
                    board.undo_move().unwrap();
                }
            }
        }
        ret
    }
}

impl<Original> Evaluator for EnhancedEvaluator<Original>
where Original: Evaluator
{
    fn evaluate(&self, side: Side, board: &Board) -> i32 {
        let mut board = board.clone();
        self.search(self.depth, side, &mut board)
    }
}
