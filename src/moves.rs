use std::cmp::{max, min};

use serde::Deserialize;

use crate::{
    bitboard::BitBoard,
    board::{Board, Color, Soldier},
    state::State,
};

type Position = (i32, i32);

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<Soldier>,
}

impl Move {
    fn from_bitboard(s: Soldier, from: Position, to: BitBoard) -> Vec<Self> {
        let mut moves = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                if to.get(x, y) {
                    if matches!(s, Soldier::Pawn) && (y == 0 || y == 7) {
                        moves.append(&mut Self::promotion_moves(from, (x as i32, y as i32)));
                    } else {
                        moves.push(Self {
                            from,
                            to: (x as i32, y as i32),
                            promotion: None,
                        });
                    }
                }
            }
        }
        moves
    }

    fn promotion_moves(from: Position, to: Position) -> Vec<Move> {
        let mut moves = Vec::new();
        for s in &[
            Soldier::Queen,
            Soldier::Rook,
            Soldier::Bishop,
            Soldier::Knight,
        ] {
            moves.push(Self {
                from,
                to,
                promotion: Some(*s),
            });
        }
        moves
    }
}

pub fn legal_moves(st: &State) -> Vec<Move> {
    // get the enemy's pseudo-legal moves without our king (so we can check attacked squares behind king)
    let opp_turn = st.turn.opposite();
    let mut opp_board = st.board.clone();
    let (kx, ky) = opp_board.remove_king(st.turn);
    let opp_moves = pseudo_legal_moves(&opp_board, opp_turn, None);

    // figure out the number of checkers, their position, and the attacked squares bitboard
    let mut checker_pos = None;
    let mut num_checkers = 0;
    let mut attacked_squares = BitBoard::new_empty();
    opp_moves.iter().enumerate().for_each(|(i, mvs)| {
        if let Some(mr) = mvs {
            let (px, py) = (i % 8, i / 8);
            let (s, _) = opp_board.get(px, py).unwrap();
            let attacks = if matches!(s, Soldier::Pawn) {
                mr.captures // only care about diagonal pawn attacks
            } else {
                mr.moves
            };
            if attacks.get(kx, ky) {
                num_checkers += 1;
                checker_pos = Some((i % 8, i / 8));
            }
            attacked_squares = attacked_squares.union(&attacks);
        }
    });

    // get our king moves based on the attacked squares, return if double check
    // if single check, create checker mask representing only legal squares in position
    let (kingside_rights, queenside_rights) = st.castling_rights_for_color();
    let king_moves = king_moves(
        &st.board,
        (kx as i32, ky as i32),
        attacked_squares,
        num_checkers > 0,
        kingside_rights,
        queenside_rights,
    );
    if num_checkers > 1 {
        return Move::from_bitboard(Soldier::King, (kx as i32, ky as i32), king_moves.moves);
    }
    let checker_mask = if num_checkers == 1 {
        get_checker_mask(&st.board, checker_pos.unwrap(), (kx, ky))
    } else {
        BitBoard::new_full() // every square is legal
    };

    // put an enemy queen where our king was to help with computing pins
    opp_board.set(kx, ky, Some((Soldier::Queen, opp_turn)));
    let pin_lines_from_king =
        sliding_moves(&opp_board, (kx as i32, ky as i32), Soldier::Queen, opp_turn);

    // for each enemy slider in line with our king, see if it pins a piece to our king
    // if so, narrow it's entry in the pinned mask to only the legal pinned moves
    let mut pinned_mask = [BitBoard::new_full(); 64];
    for (i, (piece, moves)) in st.board.iter().zip(opp_moves.iter()).enumerate() {
        let pos = (i % 8, i / 8);
        if moves.is_none() || !Board::is_sliding_piece(piece) || !Board::is_aligned(pos, (kx, ky)) {
            continue;
        }
        let mr = moves.as_ref().unwrap();
        let line = BitBoard::make_line(pos, (kx, ky)); // line joining enemy slider to king
        let pinned = line
            .intersection(&mr.moves)
            .intersection(&pin_lines_from_king.moves);

        if pinned.num_set() == 1 {
            pinned_mask[pinned.lowest_set()] = line;
        }
    }
    if let Some((ex, ey)) = st.en_passant_square {
        let dy = if st.turn == Color::White { -1 } else { 1 };
        if is_enpassant_pin_rank(&st.board, st.turn, ey as i32 + dy) {
            let col = ((ey as i32 + dy) * 8) as usize;
            pinned_mask[col + ex + 1].unset(ex, ey);
            pinned_mask[col + ex - 1].unset(ex, ey);
        }
    }

    pseudo_legal_moves(&st.board, st.turn, st.en_passant_square)
        .iter()
        .enumerate()
        .filter_map(|(i, mr)| {
            if let Some(mr) = mr {
                let pos = (i as i32 % 8, i as i32 / 8);
                let (s, _) = st.board.get(pos.0 as usize, pos.1 as usize).unwrap();
                if pos == (kx as i32, ky as i32) {
                    // we already found king moves
                    return Some(Move::from_bitboard(
                        s,
                        (kx as i32, ky as i32),
                        king_moves.moves,
                    ));
                }
                let mut checker_mask = checker_mask;
                if let Some((ex, ey)) = st.en_passant_square {
                    if num_checkers > 0 && matches!(s, Soldier::Pawn) {
                        // if we are in check from a pawn that we can
                        // enpassant, allow the enpassant square as a legal move
                        checker_mask.set(ex, ey);
                    }
                }
                let legal_moves = mr
                    .moves
                    .intersection(&checker_mask)
                    .intersection(&pinned_mask[i]);
                Some(Move::from_bitboard(s, pos, legal_moves))
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

// given a board and a position of an enemy piece that is checking the king,
// return the mask of legal moves to stop the check
fn get_checker_mask(bd: &Board, (cx, cy): (usize, usize), king_pos: (usize, usize)) -> BitBoard {
    // if checker is a
    // - pawn: can only capture, return position of pawn
    // - knight: can only capture, return position of knight
    // - slider: can capture or block, return line between slider and king
    let (s, _) = bd.get(cx, cy).unwrap();
    match s {
        Soldier::Pawn | Soldier::Knight => {
            let mut mask = BitBoard::new_empty();
            mask.set(cx, cy);
            mask
        }
        Soldier::Bishop | Soldier::Rook | Soldier::Queen => BitBoard::make_line((cx, cy), king_pos),
        _ => unreachable!(),
    }
}

fn is_enpassant_pin_rank(bd: &Board, side: Color, rank: i32) -> bool {
    // enpassant pins are the case where an enpassant capture could leave our king in check
    // the rank containing the enpassantable pawn should look like:
    // [enemy slider ... pawn pawn ... king] or [king ... pawn pawn ... enemy slider]

    let mut soldiers = Vec::new();
    let mut king_file = None;
    for file in 0..=7 {
        if let Some(p) = bd.get(file, rank as usize) {
            match p {
                (Soldier::Rook, c) | (Soldier::Queen, c) => {
                    if *c != side {
                        soldiers.push(Soldier::Rook)
                    } else {
                        soldiers.push(Soldier::Queen)
                    }
                }
                (s, c) => {
                    if *c == side && matches!(s, Soldier::King) {
                        king_file = Some(soldiers.len())
                    }
                    soldiers.push(*s)
                }
            }
        }
    }
    if king_file.is_none() {
        return false;
    }
    let king_file = king_file.unwrap();

    let high_soldiers = &soldiers[king_file..=min(king_file + 3, soldiers.len() - 1)];
    let low_soldiers = &soldiers[max(king_file - 3, 0)..=king_file];
    matches!(
        high_soldiers,
        [Soldier::King, Soldier::Pawn, Soldier::Pawn, Soldier::Rook]
    ) || matches!(
        low_soldiers,
        [Soldier::Rook, Soldier::Pawn, Soldier::Pawn, Soldier::King]
    )
}

fn sliding_piece_directions(s: &Soldier) -> Vec<(i32, i32)> {
    match s {
        Soldier::Bishop => vec![(1, 1), (1, -1), (-1, 1), (-1, -1)],
        Soldier::Rook => vec![(1, 0), (0, 1), (-1, 0), (0, -1)],
        Soldier::Queen => vec![
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
        ],
        _ => panic!("Not a sliding piece"),
    }
}

fn in_bounds((x, y): Position) -> bool {
    (0..8).contains(&x) && (0..8).contains(&y)
}

// the only reason we need this is because enpassant doesn't move to the square it captured
struct MovesResult {
    moves: BitBoard,
    captures: BitBoard,
}

fn pseudo_legal_moves(
    bd: &Board,
    side: Color,
    enpassant_square: Option<(usize, usize)>,
) -> Vec<Option<MovesResult>> {
    let mut move_sets: Vec<Option<MovesResult>> = Vec::new();
    for (i, &square) in bd.iter().enumerate() {
        let pos = (i as i32 % 8, i as i32 / 8);
        if square.is_none() || square.unwrap().1 != side {
            move_sets.push(None);
            continue;
        }
        let soldier = square.unwrap().0;
        match soldier {
            Soldier::Pawn => move_sets.push(Some(pawn_moves(bd, pos, side, enpassant_square))),
            Soldier::Knight => move_sets.push(Some(knight_moves(bd, pos))),
            Soldier::Bishop | Soldier::Rook | Soldier::Queen => {
                move_sets.push(Some(sliding_moves(bd, pos, soldier, side)))
            }
            // don't worry about castling here, it happens in legal move generation
            Soldier::King => move_sets.push(Some(king_moves(
                bd,
                pos,
                BitBoard::new_empty(),
                true,
                false,
                false,
            ))),
        }
    }
    move_sets
}

fn sliding_moves(bd: &Board, (px, py): Position, soldier: Soldier, side: Color) -> MovesResult {
    let mut captures = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    let dirs = sliding_piece_directions(&soldier);
    for (dx, dy) in dirs {
        let (mut x, mut y) = (px + dx, py + dy);
        while in_bounds((x, y)) {
            match bd.get(x as usize, y as usize) {
                Some((_, c)) if c == &side => break,
                Some(_) => {
                    captures.set(x as usize, y as usize);
                    moves.set(x as usize, y as usize);
                    break;
                }
                None => moves.set(x as usize, y as usize),
            }
            x += dx;
            y += dy;
        }
    }
    MovesResult { captures, moves }
}

fn knight_moves(bd: &Board, (px, py): Position) -> MovesResult {
    let mut captures = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    vec![
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ]
    .iter()
    .for_each(|(dx, dy)| {
        let (nx, ny) = (px + dx, py + dy);
        if in_bounds((nx, ny)) && !bd.is_same_color((px, py), (nx, ny)) {
            moves.set(nx as usize, ny as usize);
            if bd.is_enemy_color((px, py), (nx, ny)) {
                captures.set(nx as usize, ny as usize);
            }
        }
    });
    MovesResult { captures, moves }
}

fn pawn_moves(
    bd: &Board,
    (px, py): Position,
    side: Color,
    enpassant_square: Option<(usize, usize)>,
) -> MovesResult {
    let mut captures = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    let dy = if side == Color::White { 1 } else { -1 };
    let is_start = if side == Color::White {
        py == 1
    } else {
        py == 6
    };
    if bd.get(px as usize, (py + dy) as usize).is_none() {
        moves.set(px as usize, (py + dy) as usize);
        if is_start && bd.get(px as usize, (py + 2 * dy) as usize).is_none() {
            moves.set(px as usize, (py + 2 * dy) as usize);
        }
    };
    for dx in [-1, 1].iter() {
        let (nx, ny) = (px + dx, py + dy);
        if in_bounds((nx, ny)) {
            captures.set(nx as usize, ny as usize); // label diagonal as a capture so it is picked up as attacked
            if bd.is_enemy_color((px, py), (nx, ny)) {
                moves.set(nx as usize, ny as usize);
            }
        }
        if let Some((ex, ey)) = enpassant_square {
            if (nx, ny) == (ex as i32, ey as i32) {
                moves.set(nx as usize, ny as usize);
                captures.set((px + dx) as usize, py as usize);
            }
        }
    }
    MovesResult { captures, moves }
}

fn king_moves(
    bd: &Board,
    (px, py): Position,
    attacked: BitBoard,
    king_in_check: bool,
    kingside_rights: bool,
    queenside_rights: bool,
) -> MovesResult {
    let mut captures = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    vec![
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ]
    .iter()
    .for_each(|(dx, dy)| {
        let (nx, ny) = (px + dx, py + dy);
        if in_bounds((nx, ny))
            && !bd.is_same_color((px, py), (nx, ny))
            && !attacked.get(nx as usize, ny as usize)
        {
            moves.set(nx as usize, ny as usize);
            if bd.is_enemy_color((px, py), (nx, ny)) {
                captures.set(nx as usize, ny as usize);
            }
        }
    });

    if !king_in_check {
        let (px, py) = (px as usize, py as usize);
        if kingside_rights
            && bd.get(px + 1, py).is_none()
            && bd.get(px + 2, py).is_none()
            && !attacked.get(px + 1, py)
            && !attacked.get(px + 2, py)
        {
            moves.set(px + 2, py);
        }
        if queenside_rights
            && bd.get(px - 1, py).is_none()
            && bd.get(px - 2, py).is_none()
            && bd.get(px - 3, py).is_none()
            && !attacked.get(px - 1, py)
            && !attacked.get(px - 2, py)
        {
            moves.set(px - 2, py);
        }
    }

    MovesResult { captures, moves }
}
