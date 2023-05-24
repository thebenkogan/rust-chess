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
                        from: Vector::new(*fx, *fy),
                        to: Vector::new(*tx, *ty),
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

    fn count_nodes(st: &mut State, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        let mut num_nodes = 0;
        for mv in legal_moves(st) {
            st.push(mv);
            num_nodes += count_nodes(st, depth - 1);
            st.pop();
        }
        num_nodes
    }

    #[test]
    fn perft_tests() {
        let positions: Vec<PerftPosition> =
            serde_json::from_str(&fs::read_to_string("test-data/perft.json").unwrap()).unwrap();

        for position in positions {
            let mut st = State::from_fen(&position.fen);
            let num_nodes = count_nodes(&mut st, position.depth);
            assert_eq!(num_nodes, position.nodes, "FEN: {}", position.fen);
        }
    }
}
