use std::{fmt::Display, slice::Iter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Soldier {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub type Piece = (Soldier, Color);

pub struct Board([Option<Piece>; 64]);

impl Board {
    pub fn new() -> Board {
        let empty: Option<Piece> = None;
        let mut board = [empty; 64];
        for i in 8..16 {
            board[i] = Some((Soldier::Pawn, Color::White));
            board[i + 40] = Some((Soldier::Pawn, Color::Black));
        }
        board[0] = Some((Soldier::Rook, Color::White));
        board[1] = Some((Soldier::Knight, Color::White));
        board[2] = Some((Soldier::Bishop, Color::White));
        board[3] = Some((Soldier::Queen, Color::White));
        board[4] = Some((Soldier::King, Color::White));
        board[5] = Some((Soldier::Bishop, Color::White));
        board[6] = Some((Soldier::Knight, Color::White));
        board[7] = Some((Soldier::Rook, Color::White));
        board[56] = Some((Soldier::Rook, Color::Black));
        board[57] = Some((Soldier::Knight, Color::Black));
        board[58] = Some((Soldier::Bishop, Color::Black));
        board[59] = Some((Soldier::Queen, Color::Black));
        board[60] = Some((Soldier::King, Color::Black));
        board[61] = Some((Soldier::Bishop, Color::Black));
        board[62] = Some((Soldier::Knight, Color::Black));
        board[63] = Some((Soldier::Rook, Color::Black));
        Board(board)
    }

    // write a function 'iter' that gives back an iterator over references of the board pieces
    pub fn iter(&self) -> Iter<Option<Piece>> {
        self.0.iter()
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<Piece> {
        &self.0[x + y * 8]
    }

    pub fn is_same_color(&self, (x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
        match (
            self.get(x1 as usize, y1 as usize),
            self.get(x2 as usize, y2 as usize),
        ) {
            (Some((_, c1)), Some((_, c2))) => c1 == c2,
            _ => false,
        }
    }

    pub fn is_enemy_color(&self, (x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
        match (
            self.get(x1 as usize, y1 as usize),
            self.get(x2 as usize, y2 as usize),
        ) {
            (Some((_, c1)), Some((_, c2))) => c1 != c2,
            _ => false,
        }
    }

    pub fn is_same_piece(&self, (x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> bool {
        match (
            self.get(x1 as usize, y1 as usize),
            self.get(x2 as usize, y2 as usize),
        ) {
            (Some((s1, _)), Some((s2, _))) => s1 == s2,
            _ => false,
        }
    }

    pub fn is_queen(&self, (x, y): (i32, i32)) -> bool {
        matches!(self.get(x as usize, y as usize), Some((Soldier::Queen, _)))
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        let mut row = 7;
        while row >= 0 {
            for col in 0..=7 {
                if col == 0 {
                    board.push_str("\n---------------------------------------------\n");
                }
                match self.get(col, row as usize) {
                    Some((Soldier::Pawn, Color::White)) => board.push_str(" ♟ "),
                    Some((Soldier::Knight, Color::White)) => board.push_str(" ♞ "),
                    Some((Soldier::Bishop, Color::White)) => board.push_str(" ♝ "),
                    Some((Soldier::Rook, Color::White)) => board.push_str(" ♜ "),
                    Some((Soldier::Queen, Color::White)) => board.push_str(" ♛ "),
                    Some((Soldier::King, Color::White)) => board.push_str(" ♚ "),
                    Some((Soldier::Pawn, Color::Black)) => board.push_str(" ♙ "),
                    Some((Soldier::Knight, Color::Black)) => board.push_str(" ♘ "),
                    Some((Soldier::Bishop, Color::Black)) => board.push_str(" ♗ "),
                    Some((Soldier::Rook, Color::Black)) => board.push_str(" ♖ "),
                    Some((Soldier::Queen, Color::Black)) => board.push_str(" ♕ "),
                    Some((Soldier::King, Color::Black)) => board.push_str(" ♔ "),
                    None => board.push_str("   "),
                }
                if col != 7 {
                    board.push_str(" | ")
                }
            }
            row -= 1;
        }
        board.push_str("\n---------------------------------------------\n");
        write!(f, "{}", board)
    }
}
