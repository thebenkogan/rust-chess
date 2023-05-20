pub struct BitBoard(u64);

impl BitBoard {
    pub fn new() -> BitBoard {
        BitBoard(0)
    }

    pub fn set(&mut self, x: usize, y: usize) {
        self.0 |= 1 << (x + y * 8);
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.0 & (1 << (x + y * 8)) != 0
    }
}
