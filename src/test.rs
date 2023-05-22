#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::{
        board::Soldier,
        moves::{legal_moves, Move},
        state::State,
        vector::Vector,
    };
    use std::fs;

    #[derive(Deserialize)]
    struct JsonMove {
        from: (i8, i8),
        to: (i8, i8),
        promotion: Option<char>,
    }

    #[derive(Deserialize)]
    struct Position {
        fen: String,
        moves: Vec<JsonMove>,
    }

    #[test]
    fn legal_moves_test() {
        let positions: Vec<Position> =
            serde_json::from_str(&fs::read_to_string("test-data/positions.json").unwrap()).unwrap();

        for position in positions {
            let mut expected_moves: Vec<Move> = position
                .moves
                .iter()
                .map(
                    |JsonMove {
                         from: (fx, fy),
                         to: (tx, ty),
                         promotion,
                     }| Move {
                        from: Vector::from_int(*fx, *fy),
                        to: Vector::from_int(*tx, *ty),
                        promotion: match promotion {
                            Some('q') => Some(Soldier::Queen),
                            Some('r') => Some(Soldier::Rook),
                            Some('b') => Some(Soldier::Bishop),
                            Some('n') => Some(Soldier::Knight),
                            _ => None,
                        },
                    },
                )
                .collect();

            let st = State::from_fen(&position.fen);
            let mut legal_moves = legal_moves(&st);
            legal_moves.sort();
            expected_moves.sort();
            assert_eq!(legal_moves, expected_moves, "FEN: {}", position.fen);
        }
    }

    #[derive(Deserialize)]
    struct PerftPosition {
        depth: usize,
        nodes: usize,
        fen: String,
    }

    #[test]
    fn perft_tests() {
        let positions: Vec<PerftPosition> =
            serde_json::from_str(&fs::read_to_string("test-data/perft.json").unwrap()).unwrap();

        for position in positions {
            let st = State::from_fen(&position.fen);
            let depth = position.depth;
            let mut nodes = 0;
            let mut queue = vec![(st, depth)];
            while !queue.is_empty() {
                let (st, depth) = queue.pop().unwrap();
                if depth == 0 {
                    nodes += 1;
                    continue;
                }
                let moves = legal_moves(&st);
                for mv in moves {
                    let next_st = State::step(st.clone(), mv);
                    queue.push((next_st, depth - 1));
                }
            }

            assert_eq!(nodes, position.nodes, "FEN: {}", position.fen);
        }
    }
}
