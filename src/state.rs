use crate::board::{Board, Color};

pub struct State {
    pub board: Board,
    pub turn: Color,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn: Color::White,
        }
    }
}

// given state and move to make, return new state:
// - make move on (copied) board
// - find pseudo legal moves
// -
