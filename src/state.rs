use crate::{
    bitboard::BitBoard,
    board::{Board, Color},
    moves::Move,
};

pub struct State {
    pub board: Board,
    pub legal_moves: Vec<Move>,
    pub turn: Color,
    pub attacked_squares: BitBoard,
    pub last_move: Move,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            legal_moves: Vec::new(),
            turn: Color::White,
            attacked_squares: BitBoard::new(),
            last_move: Move {
                from: (-1, -1),
                to: (-1, -1),
            },
        }
    }
}
