use crate::{
    board::{Board, Color, Soldier},
    moves::Move,
    vector::Vector,
};

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone)]
pub struct State {
    pub board: Board,
    pub turn: Color,
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    pub en_passant_square: Option<Vector>,
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

    pub fn step(mut st: Self, mv: Move) -> Self {
        // remove piece from old square and move to new one
        let (mut s, c) = st.board.remove(mv.from).unwrap();
        if let Some(promotion_piece) = mv.promotion {
            s = promotion_piece;
        }
        st.board.set(mv.to, Some((s, c)));

        // update king moving castling rights and perform castling
        if s == Soldier::King {
            if st.turn == Color::White {
                st.white_castle_kingside = false;
                st.white_castle_queenside = false;
            } else {
                st.black_castle_kingside = false;
                st.black_castle_queenside = false;
            }
            match mv.to.x - mv.from.x {
                2 => {
                    st.board.remove(Vector::new(7, mv.from.y));
                    st.board
                        .set(Vector::new(5, mv.from.y), Some((Soldier::Rook, st.turn)));
                }
                -2 => {
                    st.board.remove(Vector::new(0, mv.from.y));
                    st.board
                        .set(Vector::new(3, mv.from.y), Some((Soldier::Rook, st.turn)));
                }
                _ => {}
            }
        }

        // check for rooks moved/captured to update castling rights
        if mv.from == Vector::new(7, 0) || mv.to == Vector::new(7, 0) {
            st.white_castle_kingside = false;
        }
        if mv.from == Vector::new(0, 0) || mv.to == Vector::new(0, 0) {
            st.white_castle_queenside = false;
        }
        if mv.from == Vector::new(7, 7) || mv.to == Vector::new(7, 7) {
            st.black_castle_kingside = false;
        }
        if mv.from == Vector::new(0, 7) || mv.to == Vector::new(0, 7) {
            st.black_castle_queenside = false;
        }

        // check if we took enpassant and remove the captured pawn if so
        if let Some(ev) = st.en_passant_square {
            let dy = if st.turn == Color::White { -1 } else { 1 };
            if s == Soldier::Pawn && mv.to == ev {
                st.board.remove(Vector::new(ev.x, ev.y + dy));
            }
            st.en_passant_square = None;
        }

        // update enpassant square if pawn moved 2 squares
        if s == Soldier::Pawn && (mv.from.y - mv.to.y).abs() == 2 {
            let dy = if st.turn == Color::White { -1 } else { 1 };
            st.en_passant_square = Some(Vector::new(mv.from.x, mv.to.y + dy));
        }

        // their turn now!
        st.turn = st.turn.opposite();

        st
    }
}
