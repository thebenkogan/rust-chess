use crate::{
    bitboard::BitBoard,
    board::{Board, Color, Soldier},
    state::State,
};

type Position = (i32, i32);

#[derive(Clone, Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl Move {
    fn from_bitboard(from: Position, to: BitBoard) -> Vec<Self> {
        let mut moves = Vec::new();
        for x in 0..8 {
            for y in 0..8 {
                if to.get(x, y) {
                    moves.push(Self {
                        from,
                        to: (x as i32, y as i32),
                    });
                }
            }
        }
        moves
    }
}

pub fn legal_moves(st: &State) -> Vec<Move> {
    // get the enemy's pseudo-legal moves without our king (so we can check attacked squares behind king)
    let opp_turn = st.turn.opposite();
    let mut opp_board = st.board.clone();
    let (kx, ky) = opp_board.remove_king(st.turn);
    let opp_moves = pseudo_legal_moves(&opp_board, opp_turn);

    // figure out the number of checkers, their position, and the attacked squares bitboard
    let mut checker_pos = None;
    let mut num_checkers = 0;
    let mut attacked_squares = BitBoard::new_empty();
    opp_moves.iter().enumerate().for_each(|(i, mvs)| {
        if let Some(mr) = mvs {
            if mr.moves.get(kx, ky) {
                num_checkers += 1;
                checker_pos = Some((i % 8, i / 8));
            }
            attacked_squares = attacked_squares.union(&mr.moves);
        }
    });

    // get our king moves based on the attacked squares, return if double check
    // if single check, create checker mask representing only legal squares in position
    let king_moves = king_moves(&st.board, (kx as i32, ky as i32), attacked_squares);
    if num_checkers > 1 {
        return Move::from_bitboard((kx as i32, ky as i32), king_moves.moves);
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
            pinned_mask[pinned.lowest_set()] = line.intersection(&mr.moves);
            // TODO: handle the enpassant case where num_set == 2
        }
    }

    // question: what about the enpassant case?
    // if the state told us which pawn is legal for enpassant via bitboard
    // we could check the case of two blockers, see if the intersection contains
    // the enpassantable pawn, then disallow it if this is a horizontal pin

    pseudo_legal_moves(&st.board, st.turn)
        .iter()
        .enumerate()
        .filter_map(|(i, mr)| {
            if let Some(mr) = mr {
                let pos = (i as i32 % 8, i as i32 / 8);
                let legal_moves = mr
                    .moves
                    .intersection(&checker_mask)
                    .intersection(&pinned_mask[i]);
                Some(Move::from_bitboard(pos, legal_moves))
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

fn pseudo_legal_moves(bd: &Board, side: Color) -> Vec<Option<MovesResult>> {
    let mut move_sets: Vec<Option<MovesResult>> = Vec::new();
    for (i, &square) in bd.iter().enumerate() {
        let pos = (i as i32 % 8, i as i32 / 8);
        if square.is_none() || square.unwrap().1 != side {
            move_sets.push(None);
            continue;
        }
        let soldier = square.unwrap().0;
        match soldier {
            Soldier::Pawn => move_sets.push(Some(pawn_moves(bd, pos, side))),
            Soldier::Knight => move_sets.push(Some(knight_moves(bd, pos))),
            Soldier::Bishop | Soldier::Rook | Soldier::Queen => {
                move_sets.push(Some(sliding_moves(bd, pos, soldier, side)))
            }
            Soldier::King => move_sets.push(Some(king_moves(bd, pos, BitBoard::new_empty()))),
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

fn pawn_moves(bd: &Board, (px, py): Position, side: Color) -> MovesResult {
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
        if in_bounds((nx, ny)) && bd.is_enemy_color((px, py), (nx, ny)) {
            captures.set(nx as usize, ny as usize);
            moves.set(px as usize, (py + 2 * dy) as usize);
        }
    }
    MovesResult { captures, moves }
}

fn king_moves(bd: &Board, (px, py): Position, attacked: BitBoard) -> MovesResult {
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
    MovesResult { captures, moves }
}
