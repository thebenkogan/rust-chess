use board::Soldier;
use moves::{legal_moves, Move};

use crate::state::State;

mod bitboard;
mod board;
mod fen;
mod moves;
mod state;
mod test;

fn main() {
    let init = State::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");
    let st = State::step(
        init,
        Move {
            from: (3, 6),
            to: (2, 7),
            promotion: Some(Soldier::Bishop),
        },
    );
    let st2 = State::step(
        st,
        Move {
            from: (3, 7),
            to: (3, 0),
            promotion: None,
        },
    );
    println!("{}", st2.board);
    let legal_moves = legal_moves(&st2);
    for mv in legal_moves {
        println!("{:?}", mv);
    }
}
