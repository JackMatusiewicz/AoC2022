#[repr(u8)]
#[derive(Clone, Eq, PartialEq)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    pub fn from_char(dir: char) -> Direction {
        match dir {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unreachable as we won't get bad input"),
        }
    }
}
