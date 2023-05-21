use moves::legal_moves;

use crate::state::State;

mod bitboard;
mod board;
mod fen;
mod moves;
mod state;

fn main() {
    let init = State::from_fen("4k3/1p4pp/2p5/8/q3r2Q/3p3P/1P4PK/4R3 b - -");
    let legal_moves = legal_moves(&init);
    println!("{:?}", legal_moves);
}
