#[cfg(test)]
mod tests {
    use crate::{
        moves::{legal_moves, Move},
        state::State,
    };
    use serde::Deserialize;
    use std::fs;

    #[derive(Deserialize, Debug)]
    struct Position {
        fen: String,
        moves: Vec<Move>,
    }

    #[test]
    fn legal_moves_test() {
        let positions: Vec<Position> =
            serde_json::from_str(&fs::read_to_string("test-data/positions.json").unwrap()).unwrap();

        for mut position in positions {
            let st = State::from_fen(&position.fen);
            let mut legal_moves = legal_moves(&st);
            legal_moves.sort();
            position.moves.sort();
            assert_eq!(legal_moves, position.moves, "FEN: {}", position.fen);
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
