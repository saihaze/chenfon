use ansi_term::Color;
use ansi_term::Style;

/// 陣營
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Red,
    Black,
}

impl Side {
    /// 獲取對方陣營
    pub fn other(&self) -> Side {
        match self {
            Side::Red => Side::Black,
            Side::Black => Side::Red,
        }
    }
}

/// 棋子
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

/// 獲取某處棋子相對編號（己方爲正，對方爲負）
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

/// 記錄走子歷史，以便悔棋
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UndoMoveRecord {
    from_pos: (i32, i32),
    from_piece: Option<(Side, Piece)>,
    to_pos: (i32, i32),
    to_piece: Option<(Side, Piece)>,
}

/// 檢查某位置是否在給定範圍內
fn position_inside(pos: (i32, i32), left_down: (i32, i32), right_up: (i32, i32)) -> bool {
    debug_assert!(left_down.0 <= right_up.0 && left_down.1 <= right_up.1);
    pos.0 >= left_down.0 && pos.0 <= right_up.0 && pos.1 >= left_down.1 && pos.1 <= right_up.1
}

/// 檢查某位置是否在棋盤內
fn position_inside_board(pos: (i32, i32)) -> bool {
    position_inside(pos, (0, 0), (8, 9))
}

#[derive(Debug, Clone)]
pub struct Board {
    map: [[Option<(Side, Piece)>; 10]; 9],
    undo_move_records: Vec<UndoMoveRecord>,
    move_count: u32,
    game_finished: bool,
    winner: Option<Side>,
    sum_piece: i32,
}

impl Board {
    /// 以默認開局初始化棋盤
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

        Board {
            map: map,
            undo_move_records: Vec::new(),
            move_count: 0,
            game_finished: false,
            winner: None,
            sum_piece: 0,
        }
    }

    /// 獲取 from 處棋子可到達之所有位置
    pub fn all_possible_moves(&self, from: (i32, i32)) -> Vec<(i32, i32)> {
        let fpiece = self.map[from.0 as usize][from.1 as usize];
        match fpiece {
            Some(fpiece) => {
                let mut ret = Vec::<(i32, i32)>::new();
                let side = fpiece.0;
                match fpiece.1 {
                    Piece::兵 => match side {
                        Side::Red => {
                            let pos = (from.0, from.1 + 1);
                            if position_inside_board(pos) && !self.has_friend_at(side, pos) {
                                ret.push(pos);
                            }
                            if position_inside(from, (0, 5), (8, 9)) {
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
                            if position_inside(from, (0, 0), (8, 4)) {
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
                    Piece::仕 => {
                        let left_down = match side {
                            Side::Red => (3, 0),
                            Side::Black => (3, 7),
                        };
                        let right_up = match side {
                            Side::Red => (5, 2),
                            Side::Black => (5, 9),
                        };
                        let pos = (from.0 + 1, from.1 + 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 + 1, from.1 - 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 1, from.1 + 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 1, from.1 - 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                    }
                    Piece::相 => {
                        let left_down = match side {
                            Side::Red => (0, 0),
                            Side::Black => (0, 5),
                        };
                        let right_up = match side {
                            Side::Red => (8, 4),
                            Side::Black => (8, 9),
                        };
                        let pos = (from.0 + 2, from.1 + 2);
                        let check = (from.0 + 1, from.1 + 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 + 2, from.1 - 2);
                        let check = (from.0 + 1, from.1 - 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 2, from.1 + 2);
                        let check = (from.0 - 1, from.1 + 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 2, from.1 - 2);
                        let check = (from.0 - 1, from.1 - 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                    }
                    Piece::炮 => {
                        for x in 0..(from.0) {
                            let pos = (x, from.1);
                            if !self.has_friend_at(side, pos) {
                                if self.has_piece_at(pos) {
                                    if self.piece_count(pos, from) == 3 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(pos, from) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                        for x in (from.0 + 1)..9 {
                            let pos = (x, from.1);
                            if !self.has_friend_at(side, pos) {
                                if self.has_piece_at(pos) {
                                    if self.piece_count(from, pos) == 3 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(from, pos) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                        for y in 0..(from.1) {
                            let pos = (from.0, y);
                            if !self.has_friend_at(side, pos) {
                                if self.has_piece_at(pos) {
                                    if self.piece_count(pos, from) == 3 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(pos, from) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                        for y in (from.1 + 1)..10 {
                            let pos = (from.0, y);
                            if !self.has_friend_at(side, pos) {
                                if self.has_piece_at(pos) {
                                    if self.piece_count(from, pos) == 3 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(from, pos) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                    }
                    Piece::馬 => {
                        let pos = (from.0 + 1, from.1 + 2);
                        let check = (from.0, from.1 + 1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 + 1, from.1 - 2);
                        let check = (from.0, from.1 - 1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 1, from.1 + 2);
                        let check = (from.0, from.1 + 1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 1, from.1 - 2);
                        let check = (from.0, from.1 - 1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 + 2, from.1 + 1);
                        let check = (from.0 + 1, from.1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 + 2, from.1 - 1);
                        let check = (from.0 + 1, from.1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 2, from.1 + 1);
                        let check = (from.0 - 1, from.1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 2, from.1 - 1);
                        let check = (from.0 - 1, from.1);
                        if position_inside_board(pos)
                            && !self.has_piece_at(check)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                    }
                    Piece::車 => {
                        for x in 0..(from.0) {
                            let pos = (x, from.1);
                            if !self.has_friend_at(side, pos) {
                                if self.has_enemy_at(side, pos) {
                                    if self.piece_count(pos, from) == 2 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(pos, from) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                        for x in (from.0 + 1)..9 {
                            let pos = (x, from.1);
                            if !self.has_friend_at(side, pos) {
                                if self.has_enemy_at(side, pos) {
                                    if self.piece_count(from, pos) == 2 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(from, pos) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                        for y in 0..(from.1) {
                            let pos = (from.0, y);
                            if !self.has_friend_at(side, pos) {
                                if self.has_enemy_at(side, pos) {
                                    if self.piece_count(pos, from) == 2 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(pos, from) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                        for y in (from.1 + 1)..10 {
                            let pos = (from.0, y);
                            if !self.has_friend_at(side, pos) {
                                if self.has_enemy_at(side, pos) {
                                    if self.piece_count(from, pos) == 2 {
                                        ret.push(pos);
                                    }
                                } else {
                                    if self.piece_count(from, pos) == 1 {
                                        ret.push(pos);
                                    }
                                }
                            }
                        }
                    }
                    Piece::帥 => {
                        let left_down = match side {
                            Side::Red => (3, 0),
                            Side::Black => (3, 7),
                        };
                        let right_up = match side {
                            Side::Red => (5, 2),
                            Side::Black => (5, 9),
                        };
                        let pos = (from.0, from.1 + 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0, from.1 - 1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 + 1, from.1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        let pos = (from.0 - 1, from.1);
                        if position_inside(pos, left_down, right_up)
                            && !self.has_friend_at(side, pos)
                        {
                            ret.push(pos);
                        }
                        for y in 0..10 {
                            let pos = (from.0, y);
                            if self.has_enemy_at(side, pos) {
                                let target = self.map[pos.0 as usize][pos.1 as usize].unwrap().1;
                                match target {
                                    Piece::帥 => match side {
                                        Side::Red => {
                                            if self.piece_count(from, pos) == 2 {
                                                ret.push(pos);
                                            }
                                        }
                                        Side::Black => {
                                            if self.piece_count(pos, from) == 2 {
                                                ret.push(pos);
                                            }
                                        }
                                    },
                                    _ => (),
                                }
                            }
                        }
                    }
                }
                if self.looped() {
                    debug_assert!(self.undo_move_records.len() >= 4);
                    return ret.into_iter().filter(|to| {
                        let records = &self.undo_move_records;
                        let len = records.len();
                        from != records[len - 4].from_pos
                            || to.clone() != records[len - 4].to_pos
                    }).collect();
                } else {
                    return ret;
                }
            }
            None => {
                return Vec::new();
            }
        }
    }

    /// 輸出棋局
    pub fn display(&self) {
        println!("第 {} 步後：", self.move_count);
        for y in (0..10).rev() {
            for x in 0..9 {
                let piece = self.map[x][y];
                match piece {
                    Some(piece) => {
                        let color = match piece.0 {
                            Side::Red => Color::Red,
                            Side::Black => Color::Green,
                        };
                        let str = match piece.1 {
                            Piece::兵 => "兵",
                            Piece::仕 => "仕",
                            Piece::相 => "相",
                            Piece::炮 => "炮",
                            Piece::馬 => "馬",
                            Piece::車 => "車",
                            Piece::帥 => "帥",
                        };
                        print!("{}", color.paint(str));
                    }
                    None => {
                        print!("{}", Style::new().hidden().paint("〇"));
                    }
                }
            }
            println!("");
        }
    }

    // 檢查並進行移動
    pub fn do_move(&mut self, from: (i32, i32), to: (i32, i32)) -> Result<(), ()> {
        let possible_moves = self.all_possible_moves(from);
        for possible_move in possible_moves {
            if to == possible_move {
                self.do_move_unchecked(from, to);
                return Ok(());
            }
        }
        Err(())
    }

    // 進行移動，但不做檢查
    pub fn do_move_unchecked(&mut self, from: (i32, i32), to: (i32, i32)) {
        debug_assert!(position_inside_board(from));
        debug_assert!(position_inside_board(to));
        self.move_count += 1;
        if self.has_piece_at(to) {
            if self.map[to.0 as usize][to.1 as usize].unwrap().1 == Piece::帥 {
                self.game_finished = true;
                self.winner = Some(self.map[to.0 as usize][to.1 as usize].unwrap().0.other());
            }
        }
        if self.move_count >= 2000 {
            self.game_finished = true;
        }
        self.undo_move_records.push(UndoMoveRecord {
            from_pos: from,
            from_piece: self.map[from.0 as usize][from.1 as usize],
            to_pos: to,
            to_piece: self.map[to.0 as usize][to.1 as usize],
        });
        if self.map[to.0 as usize][to.1 as usize].is_some() {
            self.sum_piece -= 1;
        }
        self.map[to.0 as usize][to.1 as usize] = self.map[from.0 as usize][from.1 as usize];
        self.map[from.0 as usize][from.1 as usize] = None;
    }

    // 遊戲是否結束
    pub fn finished(&self) -> bool {
        self.game_finished
    }

    /// 獲取地圖
    pub fn get_map(&self) -> &[[Option<(Side, Piece)>; 10]; 9] {
        &self.map
    }

    /// 獲取贏家
    pub fn get_winner(&self) -> Option<Side> {
        self.winner
    }

    /// 查詢某處是否有己方棋子
    pub fn has_friend_at(&self, side: Side, pos: (i32, i32)) -> bool {
        let piece = self.map[pos.0 as usize][pos.1 as usize];
        match piece {
            Some(piece) => piece.0 == side,
            None => false,
        }
    }

    /// 查詢某處是否有對方棋子
    pub fn has_enemy_at(&self, side: Side, pos: (i32, i32)) -> bool {
        self.has_friend_at(side.other(), pos)
    }

    /// 查詢某處是否有棋子
    pub fn has_piece_at(&self, pos: (i32, i32)) -> bool {
        match self.map[pos.0 as usize][pos.1 as usize] {
            Some(_) => true,
            None => false,
        }
    }

    /// 最後四步是否循環
    pub fn looped(&self) -> bool {
        let records = &self.undo_move_records;
        if records.len() >= 4 {
            let len = records.len();
            records[len - 4].to_piece.is_none()
                && records[len - 3].to_piece.is_none()
                && records[len - 2].to_piece.is_none()
                && records[len - 1].to_piece.is_none()
                && records[len - 4].from_pos == records[len - 2].to_pos
                && records[len - 4].to_pos == records[len - 2].from_pos
                && records[len - 3].from_pos == records[len - 1].to_pos
                && records[len - 3].to_pos == records[len - 1].from_pos
        } else {
            false
        }
    }

    // 撤銷移動
    pub fn undo_move(&mut self) -> Result<(), ()> {
        if self.undo_move_records.len() == 0 {
            Err(())
        } else {
            let record = self.undo_move_records.pop().unwrap();
            self.map[record.from_pos.0 as usize][record.from_pos.1 as usize] = record.from_piece;
            self.map[record.to_pos.0 as usize][record.to_pos.1 as usize] = record.to_piece;
            self.game_finished = false;
            self.winner = None;
            self.move_count -= 1;
            if record.to_piece.is_some() {
                self.sum_piece += 1;
            }
            Ok(())
        }
    }

    /// 獲取某範圍內棋子數目
    pub fn piece_count(&self, left_down: (i32, i32), right_up: (i32, i32)) -> i32 {
        if left_down == (0, 0) && right_up == (8, 9) {
            return self.piece_count_of_board();
        }
        let mut cnt = 0;
        for x in left_down.0..(right_up.0 + 1) {
            for y in left_down.1..(right_up.1 + 1) {
                match self.map[x as usize][y as usize] {
                    Some(_) => {
                        cnt += 1;
                    }
                    None => (),
                }
            }
        }
        cnt
    }

    /// 獲取整個棋盤中棋子個數
    pub fn piece_count_of_board(&self) -> i32 {
        self.sum_piece
    }
}
