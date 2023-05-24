use crate::{
    board::{Board, Color, Piece, Soldier},
    state::State,
    vector::Vector,
};

impl State {
    pub fn from_fen(fen: &str) -> Self {
        let mut split_fen = fen.split(' ');
        let board = get_board(split_fen.next().unwrap());
        let turn = get_active_color(split_fen.next().unwrap());
        let (
            white_castle_kingside,
            white_castle_queenside,
            black_castle_kingside,
            black_castle_queenside,
        ) = castling_rights(split_fen.next().unwrap());
        let en_passant_square = get_en_passant_square(split_fen.next().unwrap());

        State {
            board,
            turn,
            white_castle_kingside,
            white_castle_queenside,
            black_castle_kingside,
            black_castle_queenside,
            en_passant_square,
            reversions: Vec::new(),
        }
    }
}

fn get_board(piece_field: &str) -> Board {
    let mut board = Board::new();
    let mut row = 7;
    let mut col = 0;
    for c in piece_field.chars() {
        if c == '/' {
            row -= 1;
            col = 0;
            continue;
        }

        if c.is_ascii_digit() {
            col += c.to_digit(10).unwrap() as i8;
        } else {
            let piece = char_to_piece(c);
            board.set(Vector::new(col, row), Some(piece));
            col += 1;
        }
    }
    board
}

fn char_to_piece(c: char) -> Piece {
    match c {
        'r' => (Soldier::Rook, Color::Black),
        'n' => (Soldier::Knight, Color::Black),
        'b' => (Soldier::Bishop, Color::Black),
        'q' => (Soldier::Queen, Color::Black),
        'k' => (Soldier::King, Color::Black),
        'p' => (Soldier::Pawn, Color::Black),
        'R' => (Soldier::Rook, Color::White),
        'N' => (Soldier::Knight, Color::White),
        'B' => (Soldier::Bishop, Color::White),
        'Q' => (Soldier::Queen, Color::White),
        'K' => (Soldier::King, Color::White),
        'P' => (Soldier::Pawn, Color::White),
        _ => unreachable!("Invalid piece"),
    }
}

fn get_active_color(s: &str) -> Color {
    match s {
        "w" => Color::White,
        "b" => Color::Black,
        _ => unreachable!("Invalid active color"),
    }
}

fn castling_rights(s: &str) -> (bool, bool, bool, bool) {
    let mut kingside_white = false;
    let mut queenside_white = false;
    let mut kingside_black = false;
    let mut queenside_black = false;

    for c in s.chars() {
        match c {
            'K' => kingside_white = true,
            'Q' => queenside_white = true,
            'k' => kingside_black = true,
            'q' => queenside_black = true,
            '-' => break,
            _ => unreachable!("Invalid castling rights"),
        }
    }

    (
        kingside_white,
        queenside_white,
        kingside_black,
        queenside_black,
    )
}

fn get_en_passant_square(s: &str) -> Option<Vector> {
    if s == "-" {
        None
    } else {
        let mut chars = s.chars();
        let file = chars.next().unwrap() as i8 - 97;
        let rank = chars.next().unwrap().to_digit(10).unwrap() as i8 - 1;
        Some(Vector::new(file, rank))
    }
}
