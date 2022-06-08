pub mod controller;
pub mod evaluator;
pub mod game;

use std::io::stdin;

use controller::*;
use evaluator::*;
use game::*;

fn main() {
    let mut board = Board::new();
    board.display();
    let controller = AIController::new(
        EnhancedEvaluator::new(3, ShortSightedEvaluator::new())
    );
    let mut i = 0;
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let side = if i % 2 == 0 { Side::Red } else { Side::Black };
        let m = controller.decide(side, &board).unwrap();
        board.do_move(m.0, m.1).unwrap_or_else(|_| {
            println!("錯誤移動 {}, {} -> {}, {}", m.0.0, m.0.1, m.1.0, m.1.1);
        });
        board.display();
        i += 1;
    }
}
