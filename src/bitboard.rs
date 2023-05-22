use crate::vector::Vector;

#[derive(Copy, Clone, Debug)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new_empty() -> BitBoard {
        BitBoard(0)
    }

    pub fn new_full() -> BitBoard {
        BitBoard(u64::MAX)
    }

    pub fn set(&mut self, pos: Vector) {
        self.0 |= 1 << (pos.as_num());
    }

    pub fn unset(&mut self, pos: Vector) {
        self.0 &= !(1 << (pos.as_num()));
    }

    pub fn get(&self, pos: Vector) -> bool {
        self.0 & (1 << (pos.as_num())) != 0
    }

    pub fn union(&self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }

    pub fn intersection(&self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }

    // creates a bitboard with a line from (x1, y1) to but not including (x2, y2)
    pub fn make_line(p1: Vector, p2: Vector) -> BitBoard {
        let mut line = BitBoard::new_empty();
        line.set(p1);
        let dir = Vector::new((p2.x - p1.x).signum(), (p2.y - p1.y).signum());
        let mut curr = p1 + dir;
        while curr != p2 {
            line.set(curr);
            curr = curr + dir;
        }
        line
    }

    pub fn num_set(&self) -> u32 {
        self.0.count_ones()
    }

    pub fn lowest_set(&self) -> usize {
        self.0.trailing_zeros() as usize
    }
}
