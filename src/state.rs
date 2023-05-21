use crate::{
    board::{Board, Color, Soldier},
    moves::Move,
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

    pub fn step(mut st: Self, mv: Move) -> Self {
        let (fx, fy) = mv.from;
        let (tx, ty) = mv.to;

        // remove piece from old square and move to new one
        let (mut s, c) = st.board.remove(fx as usize, fy as usize).unwrap();
        if let Some(promotion_piece) = mv.promotion {
            s = promotion_piece;
        }
        st.board.set(tx as usize, ty as usize, Some((s, c)));

        // update king moving castling rights and perform castling
        if s == Soldier::King {
            if st.turn == Color::White {
                st.white_castle_kingside = false;
                st.white_castle_queenside = false;
            } else {
                st.black_castle_kingside = false;
                st.black_castle_queenside = false;
            }
            match tx - fx {
                2 => {
                    st.board.remove(7, fy as usize);
                    st.board.set(5, fy as usize, Some((Soldier::Rook, st.turn)));
                }
                -2 => {
                    st.board.remove(0, fy as usize);
                    st.board.set(3, fy as usize, Some((Soldier::Rook, st.turn)));
                }
                _ => {}
            }
        }

        // check for rooks moved/captured to update castling rights
        if (fx, fy) == (7, 0) || (tx, ty) == (7, 0) {
            st.white_castle_kingside = false;
        }
        if (fx, fy) == (0, 0) || (tx, ty) == (0, 0) {
            st.white_castle_queenside = false;
        }
        if (fx, fy) == (7, 7) || (tx, ty) == (7, 7) {
            st.black_castle_kingside = false;
        }
        if (fx, fy) == (0, 7) || (tx, ty) == (0, 7) {
            st.black_castle_queenside = false;
        }

        // check if we took enpassant and remove the captured pawn if so
        if let Some((ex, ey)) = st.en_passant_square {
            let dy = if st.turn == Color::White { -1 } else { 1 };
            if s == Soldier::Pawn && (tx as usize, ty as usize) == (ex, ey) {
                st.board.remove(ex, (ey as i32 + dy) as usize);
            }
            st.en_passant_square = None;
        }

        // update enpassant square if pawn moved 2 squares
        if s == Soldier::Pawn && (fy - ty).abs() == 2 {
            let dy = if st.turn == Color::White { -1 } else { 1 };
            st.en_passant_square = Some((fx as usize, (ty + dy) as usize));
        }

        // their turn now!
        st.turn = st.turn.opposite();

        st
    }
}
