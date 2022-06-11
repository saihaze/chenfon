pub mod ai;
pub mod controller;
pub mod game;

use ai::*;
use controller::*;
use game::*;

fn main() {
    let mut board = Board::new();
    let black_controller = RandomController::new();
    let red_controller = AIController::new(ShortSightedEvaluator::new(), 1000000);
    while !board.finished() {
        let red_move = red_controller.decide(Side::Red, &board).unwrap();
        board.do_move(red_move.0, red_move.1).unwrap();
        board.display();
        println!("分數 {}", ShortSightedEvaluator::new().evaluate(&board, Side::Red));
        if board.finished() {
            break;
        }
        let black_move = black_controller.decide(Side::Black, &board).unwrap();
        board.do_move(black_move.0, black_move.1).unwrap();
        board.display();
        println!("分數 {}", ShortSightedEvaluator::new().evaluate(&board, Side::Red));
    }
}
