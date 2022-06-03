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

    pub fn move_available(&self, from: (usize, usize), to: (usize, usize)) -> bool {
        let map = &self.m_map;
        let fpiece = &map[from.0][from.1];
        let tpiece = &map[to.0][to.1];
        match fpiece {
            Some(fpiece) => {
            }
            None => { return false; }
        }
    }
}