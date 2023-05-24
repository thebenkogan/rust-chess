use std::{fmt::Display, slice::Iter};

use crate::vector::Vector;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
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

impl Color {
    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

pub type Piece = (Soldier, Color);

#[derive(Clone)]
pub struct Board([Option<Piece>; 64]);

impl Board {
    pub fn new() -> Board {
        Board([None; 64])
    }

    pub fn iter(&self) -> Iter<Option<Piece>> {
        self.0.iter()
    }

    pub fn get(&self, pos: Vector) -> &Option<Piece> {
        &self.0[pos.as_num()]
    }

    pub fn set(&mut self, pos: Vector, piece: Option<Piece>) -> Option<Piece> {
        let prev = self.0[pos.as_num()];
        self.0[pos.as_num()] = piece;
        prev
    }

    pub fn remove(&mut self, pos: Vector) -> Option<Piece> {
        let piece = self.0[pos.as_num()];
        self.set(pos, None);
        piece
    }

    pub fn is_same_color(&self, p1: Vector, p2: Vector) -> bool {
        match (self.get(p1), self.get(p2)) {
            (Some((_, c1)), Some((_, c2))) => c1 == c2,
            _ => false,
        }
    }

    pub fn is_enemy_color(&self, p1: Vector, p2: Vector) -> bool {
        match (self.get(p1), self.get(p2)) {
            (Some((_, c1)), Some((_, c2))) => c1 != c2,
            _ => false,
        }
    }

    pub fn remove_king(&mut self, color: Color) -> Vector {
        for v in Vector::board_pos_iter() {
            if let Some((Soldier::King, c)) = self.get(v) {
                if *c == color {
                    self.set(v, None);
                    return v;
                }
            }
        }
        unreachable!("King not on board")
    }

    pub fn is_sliding_piece(p: &Option<Piece>) -> bool {
        matches!(
            p,
            Some((Soldier::Bishop, _)) | Some((Soldier::Rook, _)) | Some((Soldier::Queen, _))
        )
    }

    pub fn is_aligned(p1: Vector, p2: Vector) -> bool {
        p1.x == p2.x || p1.y == p2.y || (p1.x - p2.x).abs() == (p1.y - p2.y).abs()
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
                match self.get(Vector::new(col, row)) {
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
