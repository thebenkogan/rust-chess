use crate::board::{Board, Color};

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct State {
    pub board: Board,
    pub turn: Color,
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    pub en_passant_square: Option<(usize, usize)>,
}

impl State {
    pub fn new() -> Self {
        Self::from_fen(STARTING_FEN)
    }

    pub fn castling_rights_for_color(&self) -> (bool, bool) {
        match self.turn {
            Color::White => (self.white_castle_kingside, self.white_castle_queenside),
            Color::Black => (self.black_castle_kingside, self.black_castle_queenside),
        }
    }
}
