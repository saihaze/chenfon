pub mod controller;
pub mod game;

use std::io::stdin;

use controller::*;
use game::*;

fn main() {
    let mut board = Board::new();
    board.display();
    let controller = RandomController::new();
    let mut i = 0;
    loop {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        let side = if i % 2 == 0 {
            Side::Red
        } else {
            Side::Black
        };
        let m = controller.decide(side, &board).unwrap();
        board.do_move(m.0, m.1).unwrap();
        board.display();
        i += 1;
    }
}
