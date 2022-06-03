#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Red,
    Black,
}

impl Side {
    pub fn other(&self) -> Side {
        match self {
            Side::Red => Side::Black,
            Side::Black => Side::Red,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    兵 = 1,
    仕 = 2,
    相 = 3,
    炮 = 4,
    馬 = 5,
    車 = 6,
    帥 = 7,
}

pub fn piece_relative_id(side: Side, piece: Option<(Side, Piece)>) -> i32 {
    match piece {
        Some(piece) => {
            let abs_value = piece.1 as i32;
            if side == piece.0 {
                abs_value
            } else {
                -abs_value
            }
        }
        None => 0,
    }
}

pub fn piece_relative_score(side: Side, piece: Option<(Side, Piece)>) -> i32 {
    match piece {
        Some(piece) => {
            let abs_value = match piece.1 {
                Piece::兵 => 1,
                Piece::仕 => 3,
                Piece::相 => 3,
                Piece::炮 => 5,
                Piece::馬 => 5,
                Piece::車 => 10,
                Piece::帥 => 100,
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

#[derive(Debug, Clone)]
pub struct Board {
    m_map: [[Option<(Side, Piece)>; 10]; 9],
}

fn position_inside(
    pos: (usize, usize),
    left_down: (usize, usize),
    right_up: (usize, usize),
) -> bool {
    pos.0 >= left_down.0 && pos.0 <= right_up.0 && pos.1 >= left_down.1 && pos.1 <= right_up.1
}

fn position_inside_board(pos: (usize, usize)) -> bool {
    position_inside(pos, (0, 0), (8, 9))
}

impl Board {
    pub fn new() -> Board {
        let mut map: [[Option<(Side, Piece)>; 10]; 9] = [[None; 10]; 9];
        map[0][0] = Some((Side::Red, Piece::車));
        map[1][0] = Some((Side::Red, Piece::馬));
        map[2][0] = Some((Side::Red, Piece::相));
        map[3][0] = Some((Side::Red, Piece::仕));
        map[4][0] = Some((Side::Red, Piece::帥));
        map[5][0] = Some((Side::Red, Piece::仕));
        map[6][0] = Some((Side::Red, Piece::相));
        map[7][0] = Some((Side::Red, Piece::馬));
        map[8][0] = Some((Side::Red, Piece::車));
        map[1][2] = Some((Side::Red, Piece::炮));
        map[7][2] = Some((Side::Red, Piece::炮));
        map[0][3] = Some((Side::Red, Piece::兵));
        map[2][3] = Some((Side::Red, Piece::兵));
        map[4][3] = Some((Side::Red, Piece::兵));
        map[6][3] = Some((Side::Red, Piece::兵));
        map[8][3] = Some((Side::Red, Piece::兵));

        map[0][9] = Some((Side::Black, Piece::車));
        map[1][9] = Some((Side::Black, Piece::馬));
        map[2][9] = Some((Side::Black, Piece::相));
        map[3][9] = Some((Side::Black, Piece::仕));
        map[4][9] = Some((Side::Black, Piece::帥));
        map[5][9] = Some((Side::Black, Piece::仕));
        map[6][9] = Some((Side::Black, Piece::相));
        map[7][9] = Some((Side::Black, Piece::馬));
        map[8][9] = Some((Side::Black, Piece::車));
        map[1][7] = Some((Side::Black, Piece::炮));
        map[7][7] = Some((Side::Black, Piece::炮));
        map[0][6] = Some((Side::Black, Piece::兵));
        map[2][6] = Some((Side::Black, Piece::兵));
        map[4][6] = Some((Side::Black, Piece::兵));
        map[6][6] = Some((Side::Black, Piece::兵));
        map[8][6] = Some((Side::Black, Piece::兵));

        Board { m_map: map }
    }

    fn has_friend_at(&self, side: Side, pos: (usize, usize)) -> bool {
        let piece = self.m_map[pos.0][pos.1];
        match piece {
            Some(piece) => piece.0 == side,
            None => false,
        }
    }

    pub fn all_possible_moves(&self, from: (usize, usize)) -> Vec<(usize, usize)> {
        let fpiece = self.m_map[from.0][from.1];
        match fpiece {
            Some(fpiece) => {
                let mut ret = Vec::<(usize, usize)>::new();
                let side = fpiece.0;
                match fpiece.1 {
                    Piece::兵 => match side {
                        Side::Red => {
                            let pos = (from.0, from.1 + 1);
                            if position_inside_board(pos) && !self.has_friend_at(side, pos) {
                                ret.push(pos);
                            }
                            if position_inside(pos, (0, 5), (8, 9)) {
                                let pos = (from.0 + 1, from.1);
                                if position_inside_board(pos) && !self.has_friend_at(side, pos) {
                                    ret.push(pos);
                                }
                                let pos = (from.0 - 1, from.1);
                                if position_inside_board(pos) && !self.has_friend_at(side, pos) {
                                    ret.push(pos);
                                }
                            }
                        }
                        Side::Black => {
                            let pos = (from.0, from.1 - 1);
                            if position_inside_board(pos) && !self.has_friend_at(side, pos) {
                                ret.push(pos);
                            }
                            if position_inside(pos, (0, 0), (8, 4)) {
                                let pos = (from.0 + 1, from.1);
                                if position_inside_board(pos) && !self.has_friend_at(side, pos) {
                                    ret.push(pos);
                                }
                                let pos = (from.0 - 1, from.1);
                                if position_inside_board(pos) && !self.has_friend_at(side, pos) {
                                    ret.push(pos);
                                }
                            }
                        }
                    },
                    Piece::仕 => {}
                    Piece::相 => {}
                    Piece::炮 => {}
                    Piece::馬 => {}
                    Piece::車 => {}
                    Piece::帥 => {}
                }
                return ret;
            }
            None => {
                return Vec::new();
            }
        }
    }
}
