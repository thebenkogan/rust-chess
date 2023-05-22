use std::cmp::{max, min};

use crate::{
    bitboard::BitBoard,
    board::{Board, Color, Soldier},
    state::State,
    vector::Vector,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Move {
    pub from: Vector,
    pub to: Vector,
    pub promotion: Option<Soldier>,
}

impl Move {
    fn from_bitboard(s: Soldier, from: Vector, to: BitBoard) -> Vec<Self> {
        let mut moves = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                let pos = Vector::from_usize(x, y);
                if to.get(pos) {
                    if matches!(s, Soldier::Pawn) && (y == 0 || y == 7) {
                        moves.append(&mut Self::promotion_moves(from, pos));
                    } else {
                        moves.push(Self {
                            from,
                            to: Vector::from_usize(x, y),
                            promotion: None,
                        });
                    }
                }
            }
        }
        moves
    }

    fn promotion_moves(from: Vector, to: Vector) -> Vec<Move> {
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
    let kv = opp_board.remove_king(st.turn);
    let opp_moves = pseudo_legal_moves(&opp_board, opp_turn, None);

    // figure out the number of checkers, their position, and the attacked squares bitboard
    let mut checker_pos = None;
    let mut num_checkers = 0;
    let mut attacked_squares = BitBoard::new_empty();
    opp_moves.iter().enumerate().for_each(|(i, mvs)| {
        if let Some(mr) = mvs {
            let pv = Vector::from_num(i);
            let (s, _) = opp_board.get(pv).unwrap();
            let attacks = if matches!(s, Soldier::Pawn) {
                mr.attacks // only care about diagonal pawn attacks
            } else {
                mr.moves.union(&mr.attacks)
            };
            if attacks.get(kv) {
                num_checkers += 1;
                checker_pos = Some(pv);
            }
            attacked_squares = attacked_squares.union(&attacks);
        }
    });

    // get our king moves based on the attacked squares, return if double check
    // if single check, create checker mask representing only legal squares in position
    let (kingside_rights, queenside_rights) = st.castling_rights_for_color();
    let king_moves = king_moves(
        &st.board,
        kv,
        attacked_squares,
        num_checkers > 0,
        kingside_rights,
        queenside_rights,
    );
    if num_checkers > 1 {
        return Move::from_bitboard(Soldier::King, kv, king_moves.moves);
    }
    let checker_mask = if num_checkers == 1 {
        get_checker_mask(&st.board, checker_pos.unwrap(), kv)
    } else {
        BitBoard::new_full() // every square is legal
    };

    // put an enemy queen where our king was to help with computing pins
    opp_board.set(kv, Some((Soldier::Queen, opp_turn)));
    let pin_lines_from_king = sliding_moves(&opp_board, kv, Soldier::Queen, opp_turn);

    // for each enemy slider in line with our king, see if it pins a piece to our king
    // if so, narrow it's entry in the pinned mask to only the legal pinned moves
    let mut pinned_mask = [BitBoard::new_full(); 64];
    for (i, (piece, moves)) in st.board.iter().zip(opp_moves.iter()).enumerate() {
        let pos = Vector::from_num(i);
        if moves.is_none() || !Board::is_sliding_piece(piece) || !Board::is_aligned(pos, kv) {
            continue;
        }
        let mr = moves.as_ref().unwrap();
        let line = BitBoard::make_line(pos, kv); // line joining enemy slider to king
        let pinned = line
            .intersection(&mr.moves)
            .intersection(&pin_lines_from_king.moves);

        if pinned.num_set() == 1 {
            pinned_mask[pinned.lowest_set()] = line;
        }
    }
    if let Some(ev) = st.en_passant_square {
        let dy = if st.turn == Color::White { -1 } else { 1 };
        if is_enpassant_pin_rank(&st.board, st.turn, ev.y + dy) {
            let col = ((ev.y + dy) * 8) as usize;
            pinned_mask[col + ev.x as usize + 1].unset(ev);
            pinned_mask[col + ev.x as usize - 1].unset(ev);
        }
    }

    pseudo_legal_moves(&st.board, st.turn, st.en_passant_square)
        .iter()
        .enumerate()
        .filter_map(|(i, mr)| {
            if let Some(mr) = mr {
                let pos = Vector::from_num(i);
                let (s, _) = st.board.get(pos).unwrap();
                if pos == kv {
                    // we already found king moves
                    return Some(Move::from_bitboard(s, kv, king_moves.moves));
                }
                let mut checker_mask = checker_mask;
                if let Some(ev) = st.en_passant_square {
                    if let Some(cv) = checker_pos {
                        let (cs, _) = st.board.get(cv).unwrap();
                        if matches!(cs, Soldier::Pawn) && matches!(s, Soldier::Pawn) {
                            // if we are in check from a pawn that we can
                            // enpassant, allow the enpassant square as a legal move
                            checker_mask.set(ev);
                        }
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
fn get_checker_mask(bd: &Board, checker_pos: Vector, king_pos: Vector) -> BitBoard {
    // if checker is a
    // - pawn: can only capture, return position of pawn
    // - knight: can only capture, return position of knight
    // - slider: can capture or block, return line between slider and king
    let (s, _) = bd.get(checker_pos).unwrap();
    match s {
        Soldier::Pawn | Soldier::Knight => {
            let mut mask = BitBoard::new_empty();
            mask.set(checker_pos);
            mask
        }
        Soldier::Bishop | Soldier::Rook | Soldier::Queen => {
            BitBoard::make_line(checker_pos, king_pos)
        }
        _ => unreachable!(),
    }
}

fn is_enpassant_pin_rank(bd: &Board, side: Color, rank: i8) -> bool {
    // enpassant pins are the case where an enpassant capture could leave our king in check
    // the rank containing the enpassantable pawn should look like:
    // [enemy slider ... pawn pawn ... king] or [king ... pawn pawn ... enemy slider]

    let mut soldiers = Vec::new();
    let mut king_file = None;
    for file in 0..=7 {
        if let Some(p) = bd.get(Vector::from_int(file, rank)) {
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
    let low_soldiers = &soldiers[max(king_file as i32 - 3, 0) as usize..=king_file];
    matches!(
        high_soldiers,
        [Soldier::King, Soldier::Pawn, Soldier::Pawn, Soldier::Rook]
    ) || matches!(
        low_soldiers,
        [Soldier::Rook, Soldier::Pawn, Soldier::Pawn, Soldier::King]
    )
}

fn sliding_piece_directions(s: &Soldier) -> Vec<Vector> {
    match s {
        Soldier::Bishop => Vector::bishop_dirs(),
        Soldier::Rook => Vector::rook_dirs(),
        Soldier::Queen => Vector::queen_dirs(),
        _ => panic!("Not a sliding piece"),
    }
}

struct MovesResult {
    moves: BitBoard,
    attacks: BitBoard, // squares attacked (same-color capture, diagonal pawn, etc.)
}

fn pseudo_legal_moves(
    bd: &Board,
    side: Color,
    enpassant_square: Option<Vector>,
) -> Vec<Option<MovesResult>> {
    let mut move_sets: Vec<Option<MovesResult>> = Vec::new();
    for (i, &square) in bd.iter().enumerate() {
        let pos = Vector::from_num(i);
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

fn sliding_moves(bd: &Board, pos: Vector, soldier: Soldier, side: Color) -> MovesResult {
    let mut attacks = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    let dirs = sliding_piece_directions(&soldier);
    for dir in dirs {
        let mut curr = pos + dir;
        while curr.in_bounds() {
            attacks.set(curr);
            match bd.get(curr) {
                Some((_, c)) if c == &side => break,
                Some(_) => {
                    moves.set(curr);
                    break;
                }
                None => moves.set(curr),
            }
            curr = curr + dir;
        }
    }
    MovesResult { attacks, moves }
}

fn knight_moves(bd: &Board, pos: Vector) -> MovesResult {
    let mut attacks = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    Vector::knight_dirs().iter().for_each(|dir| {
        let offset = pos + *dir;
        if offset.in_bounds() {
            attacks.set(offset);
            if !bd.is_same_color(pos, offset) {
                moves.set(offset);
            }
        }
    });
    MovesResult { attacks, moves }
}

fn pawn_moves(
    bd: &Board,
    pos: Vector,
    side: Color,
    enpassant_square: Option<Vector>,
) -> MovesResult {
    let mut attacks = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    let push_dir = Vector::from_int(0, if side == Color::White { 1 } else { -1 });
    let is_start = if side == Color::White {
        pos.y == 1
    } else {
        pos.y == 6
    };
    if bd.get(pos + push_dir).is_none() {
        moves.set(pos + push_dir);
        if is_start && bd.get(pos + push_dir + push_dir).is_none() {
            moves.set(pos + push_dir + push_dir);
        }
    };
    for dx in [-1, 1].iter() {
        let dir = Vector::from_int(*dx, push_dir.y);
        let offset = pos + dir;
        if offset.in_bounds() {
            attacks.set(offset); // label diagonal as a capture so it is picked up as attacked
            if bd.is_enemy_color(pos, offset) {
                moves.set(offset);
            }
        }
        if let Some(ev) = enpassant_square {
            if offset == ev {
                moves.set(offset);
                attacks.set(Vector::from_int(pos.x + dx, pos.y));
            }
        }
    }
    MovesResult { attacks, moves }
}

fn king_moves(
    bd: &Board,
    pos: Vector,
    attacked: BitBoard,
    king_in_check: bool,
    kingside_rights: bool,
    queenside_rights: bool,
) -> MovesResult {
    let mut attacks = BitBoard::new_empty();
    let mut moves = BitBoard::new_empty();
    Vector::king_dirs().iter().for_each(|dir| {
        let offset = pos + *dir;
        if offset.in_bounds() {
            attacks.set(offset);
            if !bd.is_same_color(pos, offset) && !attacked.get(offset) {
                moves.set(offset);
            }
        }
    });

    if !king_in_check {
        if kingside_rights
            && bd.get(Vector::from_int(pos.x + 1, pos.y)).is_none()
            && bd.get(Vector::from_int(pos.x + 2, pos.y)).is_none()
            && !attacked.get(Vector::from_int(pos.x + 1, pos.y))
            && !attacked.get(Vector::from_int(pos.x + 2, pos.y))
        {
            moves.set(Vector::from_int(pos.x + 2, pos.y));
        }
        if queenside_rights
            && bd.get(Vector::from_int(pos.x - 1, pos.y)).is_none()
            && bd.get(Vector::from_int(pos.x - 2, pos.y)).is_none()
            && bd.get(Vector::from_int(pos.x - 3, pos.y)).is_none()
            && !attacked.get(Vector::from_int(pos.x - 1, pos.y))
            && !attacked.get(Vector::from_int(pos.x - 2, pos.y))
        {
            moves.set(Vector::from_int(pos.x - 2, pos.y));
        }
    }

    MovesResult { attacks, moves }
}
