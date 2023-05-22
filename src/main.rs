use moves::legal_moves;

use crate::state::State;

mod bitboard;
mod board;
mod fen;
mod moves;
mod state;
mod test;
mod vector;

fn main() {
    let init = State::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    println!("{}", init.board);
    let legal_moves = legal_moves(&init);
    for mv in legal_moves {
        println!("{:?}", mv);
    }
}
