use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vector {
    pub x: i8,
    pub y: i8,
}

impl Vector {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn from_num(n: usize) -> Self {
        Self {
            x: (n % 8) as i8,
            y: (n / 8) as i8,
        }
    }

    pub fn as_num(&self) -> usize {
        (self.x as usize) + (self.y as usize) * 8
    }

    pub fn in_bounds(&self) -> bool {
        self.x >= 0 && self.x < 8 && self.y >= 0 && self.y < 8
    }

    pub fn board_pos_iter() -> impl Iterator<Item = Self> {
        (0..64).map(Self::from_num)
    }

    pub fn knight_dirs() -> Vec<Self> {
        vec![
            Self::new(1, 2),
            Self::new(2, 1),
            Self::new(2, -1),
            Self::new(1, -2),
            Self::new(-1, -2),
            Self::new(-2, -1),
            Self::new(-2, 1),
            Self::new(-1, 2),
        ]
    }

    pub fn rook_dirs() -> Vec<Self> {
        vec![
            Self::new(1, 0),
            Self::new(0, 1),
            Self::new(-1, 0),
            Self::new(0, -1),
        ]
    }

    pub fn bishop_dirs() -> Vec<Self> {
        vec![
            Self::new(1, 1),
            Self::new(-1, 1),
            Self::new(-1, -1),
            Self::new(1, -1),
        ]
    }

    pub fn queen_dirs() -> Vec<Self> {
        vec![
            Self::new(1, 0),
            Self::new(1, 1),
            Self::new(0, 1),
            Self::new(-1, 1),
            Self::new(-1, 0),
            Self::new(-1, -1),
            Self::new(0, -1),
            Self::new(1, -1),
        ]
    }

    pub fn king_dirs() -> Vec<Self> {
        Self::queen_dirs()
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
