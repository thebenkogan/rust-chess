use crate::{
    board::{Board, Color, Piece, Soldier},
    moves::Move,
    vector::Vector,
};

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct State {
    pub board: Board,
    pub turn: Color,
    pub white_castle_kingside: bool,
    pub white_castle_queenside: bool,
    pub black_castle_kingside: bool,
    pub black_castle_queenside: bool,
    pub en_passant_square: Option<Vector>,
    // stack that defines how to return to the previous state
    pub reversions: Vec<Reversion>,
}

pub struct Reversion {
    mv: Move,
    captured_piece: Option<Piece>,
    white_castle_kingside: bool,
    white_castle_queenside: bool,
    black_castle_kingside: bool,
    black_castle_queenside: bool,
    en_passant_square: Option<Vector>,
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

    pub fn push(&mut self, mv: Move) {
        // remove piece from old square and move to new one
        let (mut s, c) = self.board.remove(mv.from).unwrap();
        if let Some(promotion_piece) = mv.promotion {
            s = promotion_piece;
        }
        let captured_piece = self.board.set(mv.to, Some((s, c)));

        // save how to revert this move
        let mut reversion = Reversion {
            mv,
            captured_piece,
            white_castle_kingside: self.white_castle_kingside,
            white_castle_queenside: self.white_castle_queenside,
            black_castle_kingside: self.black_castle_kingside,
            black_castle_queenside: self.black_castle_queenside,
            en_passant_square: self.en_passant_square,
        };

        // update king moving castling rights and perform castling
        if s == Soldier::King {
            if self.turn == Color::White {
                self.white_castle_kingside = false;
                self.white_castle_queenside = false;
            } else {
                self.black_castle_kingside = false;
                self.black_castle_queenside = false;
            }
            match mv.to.x - mv.from.x {
                2 => {
                    self.board.remove(Vector::new(7, mv.from.y));
                    self.board
                        .set(Vector::new(5, mv.from.y), Some((Soldier::Rook, self.turn)));
                }
                -2 => {
                    self.board.remove(Vector::new(0, mv.from.y));
                    self.board
                        .set(Vector::new(3, mv.from.y), Some((Soldier::Rook, self.turn)));
                }
                _ => {}
            }
        }

        // check for rooks moved/captured to update castling rights
        if mv.from == Vector::new(7, 0) || mv.to == Vector::new(7, 0) {
            self.white_castle_kingside = false;
        }
        if mv.from == Vector::new(0, 0) || mv.to == Vector::new(0, 0) {
            self.white_castle_queenside = false;
        }
        if mv.from == Vector::new(7, 7) || mv.to == Vector::new(7, 7) {
            self.black_castle_kingside = false;
        }
        if mv.from == Vector::new(0, 7) || mv.to == Vector::new(0, 7) {
            self.black_castle_queenside = false;
        }

        // check if we took enpassant and remove the captured pawn if so
        if let Some(ev) = self.en_passant_square {
            let dy = if self.turn == Color::White { -1 } else { 1 };
            if s == Soldier::Pawn && mv.to == ev {
                reversion.captured_piece = self.board.remove(Vector::new(ev.x, ev.y + dy));
            }
            self.en_passant_square = None;
        }

        // update enpassant square if pawn moved 2 squares
        if s == Soldier::Pawn && (mv.from.y - mv.to.y).abs() == 2 {
            let dy = if self.turn == Color::White { -1 } else { 1 };
            self.en_passant_square = Some(Vector::new(mv.from.x, mv.to.y + dy));
        }

        // their turn now!
        self.turn = self.turn.opposite();

        self.reversions.push(reversion);
    }

    pub fn pop(&mut self) {
        assert!(!self.reversions.is_empty(), "pop from root state");
        let reversion = self.reversions.pop().unwrap();

        self.white_castle_kingside = reversion.white_castle_kingside;
        self.white_castle_queenside = reversion.white_castle_queenside;
        self.black_castle_kingside = reversion.black_castle_kingside;
        self.black_castle_queenside = reversion.black_castle_queenside;
        self.en_passant_square = reversion.en_passant_square;
        self.turn = self.turn.opposite();

        let (mut s, c) = self.board.remove(reversion.mv.to).unwrap();
        if reversion.mv.promotion.is_some() {
            s = Soldier::Pawn;
        }
        self.board.set(reversion.mv.from, Some((s, c)));

        if let Some(ev) = reversion.en_passant_square {
            let dy = if self.turn == Color::White { -1 } else { 1 };
            if s == Soldier::Pawn && reversion.mv.to == ev {
                self.board
                    .set(Vector::new(ev.x, ev.y + dy), reversion.captured_piece);
            } else {
                self.board.set(reversion.mv.to, reversion.captured_piece);
            }
        } else {
            self.board.set(reversion.mv.to, reversion.captured_piece);
        }

        if s == Soldier::King {
            match reversion.mv.to.x - reversion.mv.from.x {
                2 => {
                    self.board.remove(Vector::new(5, reversion.mv.from.y));
                    self.board.set(
                        Vector::new(7, reversion.mv.from.y),
                        Some((Soldier::Rook, c)),
                    );
                }
                -2 => {
                    self.board.remove(Vector::new(3, reversion.mv.from.y));
                    self.board.set(
                        Vector::new(0, reversion.mv.from.y),
                        Some((Soldier::Rook, c)),
                    );
                }
                _ => {}
            }
        }
    }
}
