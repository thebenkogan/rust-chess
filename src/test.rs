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
