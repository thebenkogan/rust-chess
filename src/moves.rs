use crate::{
    board::{Color, Soldier},
    state::State,
};

type Position = (i32, i32);

#[derive(Clone, Debug)]
pub struct Move {
    pub from: Position,
    pub to: Position,
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

pub fn legal_moves(st: &State) -> Vec<Move> {
    let mut move_sets: Vec<Move> = Vec::new();
    for (i, &square) in st.board.iter().enumerate() {
        let pos = (i as i32 % 8, i as i32 / 8);
        if square.is_none() || square.unwrap().1 != st.turn {
            continue;
        }
        let soldier = square.unwrap().0;
        match soldier {
            Soldier::Pawn => move_sets.append(&mut pawn_moves(st, pos)),
            Soldier::Knight => move_sets.append(&mut knight_moves(st, pos)),
            Soldier::Bishop | Soldier::Rook | Soldier::Queen => move_sets.append(
                &mut sliding_moves(st, pos, sliding_piece_directions(&soldier)),
            ),
            Soldier::King => move_sets.append(&mut king_moves(st, pos)),
        }
    }
    move_sets
}

fn sliding_moves(st: &State, (px, py): Position, dirs: Vec<(i32, i32)>) -> Vec<Move> {
    let mut move_set: Vec<Move> = Vec::new();
    for (dx, dy) in dirs {
        let (mut x, mut y) = (px + dx, py + dy);
        while in_bounds((x, y)) {
            match st.board.get(x as usize, y as usize) {
                Some((_, c)) if c == &st.turn => break,
                Some(_) => {
                    move_set.push(Move {
                        from: (px, py),
                        to: (x, y),
                    });
                    break;
                }
                None => {
                    move_set.push(Move {
                        from: (px, py),
                        to: (x, y),
                    });
                }
            }
            x += dx;
            y += dy;
        }
    }
    move_set
}

fn knight_moves(st: &State, (px, py): Position) -> Vec<Move> {
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
    .filter_map(|(dx, dy)| {
        let (nx, ny) = (px + dx, py + dy);
        if in_bounds((nx, ny)) && !st.board.is_same_color((px, py), (nx, ny)) {
            Some(Move {
                from: (px, py),
                to: (nx, ny),
            })
        } else {
            None
        }
    })
    .collect()
}

fn pawn_moves(st: &State, (px, py): Position) -> Vec<Move> {
    let mut move_set: Vec<Move> = Vec::new();
    let dy = if st.turn == Color::White { 1 } else { -1 };
    let is_start = if st.turn == Color::White {
        py == 1
    } else {
        py == 6
    };
    if st.board.get(px as usize, (py + dy) as usize).is_none() {
        move_set.push(Move {
            from: (px, py),
            to: (px, py + dy),
        });
        if is_start && st.board.get(px as usize, (py + 2 * dy) as usize).is_none() {
            move_set.push(Move {
                from: (px, py),
                to: (px, py + 2 * dy),
            });
        }
    };
    for dx in [-1, 1].iter() {
        let (nx, ny) = (px + dx, py + dy);
        if in_bounds((nx, ny)) && st.board.is_enemy_color((px, py), (nx, ny)) {
            move_set.push(Move {
                from: (px, py),
                to: (nx, ny),
            });
        }
    }
    move_set
}

fn king_moves(st: &State, (px, py): Position) -> Vec<Move> {
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
    .filter_map(|(dx, dy)| {
        let (nx, ny) = (px + dx, py + dy);
        if in_bounds((nx, ny)) && !st.board.is_same_color((px, py), (nx, ny)) {
            Some(Move {
                from: (px, py),
                to: (nx, ny),
            })
        } else {
            None
        }
    })
    .collect()
}
