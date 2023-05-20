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
}
