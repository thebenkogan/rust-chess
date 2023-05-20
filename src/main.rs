use moves::legal_moves;

use crate::state::State;

mod bitboard;
mod board;
mod moves;
mod state;

fn main() {
    let init = State::new();
    let legal_moves = legal_moves(&init);
    println!("{:?}", legal_moves);
}
