use moves::legal_moves;

use crate::state::State;

mod bitboard;
mod board;
mod moves;
mod state;

fn main() {
    let init = State::new();
    let moves = legal_moves(&init);
    println!("{:?}", moves);
}
