#[derive(Copy, Clone, Debug)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new_empty() -> BitBoard {
        BitBoard(0)
    }

    pub fn new_full() -> BitBoard {
        BitBoard(u64::MAX)
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.0 |= 1 << (x + y * 8);
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.0 & (1 << (x + y * 8)) != 0
    }

    pub fn union(&self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }

    pub fn intersection(&self, other: &BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }

    // creates a bitboard with a line from (x1, y1) to but not including (x2, y2)
    pub fn make_line((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> BitBoard {
        let mut line = BitBoard::new_empty();
        line.set(x1, y1);
        let dx = (x2 as i32 - x1 as i32).signum();
        let dy = (y2 as i32 - y1 as i32).signum();
        let mut x = x1 as i32 + dx;
        let mut y = y1 as i32 + dy;
        while x != x2 as i32 || y != y2 as i32 {
            line.set(x as usize, y as usize);
            x += dx;
            y += dy;
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
